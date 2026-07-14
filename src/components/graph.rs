use crate::palette::Palette;
use iced::{
    Color, Element, Length, Point, Rectangle, Renderer, Theme, mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path},
};

struct Graph {
    data: Vec<f32>,
    maximum_value: f32,
    background_color: fn(&Palette) -> Color,
    filled_color: fn(&Palette) -> Color,
}

pub fn view<Message: 'static>(
    data: Vec<f32>,
    maximum_value: f32,
    height: f32,
    background_color: fn(&Palette) -> Color,
    filled_color: fn(&Palette) -> Color,
) -> Element<'static, Message> {
    Canvas::new(Graph {
        data,
        maximum_value,
        background_color,
        filled_color,
    })
    .width(Length::Fill)
    .height(Length::Fixed(height))
    .into()
}

impl<Message> canvas::Program<Message> for Graph {
    type State = ();

    fn draw(
        &self,
        _: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill_rectangle(
            iced::Point::ORIGIN,
            bounds.size(),
            (self.background_color)(Palette::from(theme)),
        );

        let width = bounds.width;
        let height = bounds.height;

        if self.data.is_empty() {
            return vec![frame.into_geometry()];
        }

        let gap = width / ((self.data.len() - 1) as f32);

        let points: Vec<iced::Point> = self
            .data
            .iter()
            .enumerate()
            .map(|(index, &value)| {
                let x = index as f32 * gap;
                let y = height - (value / self.maximum_value * height);
                iced::Point::new(x, y)
            })
            .collect();

        let filled_area = Path::new(|builder| {
            builder.move_to(Point::new(0.0, height));
            builder.line_to(points[0]);

            for i in 0..points.len() - 1 {
                let point_1 = points[i];
                let point_2 = points[i + 1];

                let mid_point =
                    Point::new((point_1.x + point_2.x) / 2.0, (point_1.y + point_2.y) / 2.0);
                builder.quadratic_curve_to(point_1, mid_point);
            }

            if let Some(&last_point) = points.last() {
                builder.line_to(last_point);
            }
            builder.line_to(Point::new(width, height));
        });

        frame.fill(&filled_area, (self.filled_color)(Palette::from(theme)));
        vec![frame.into_geometry()]
    }
}
