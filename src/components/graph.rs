use crate::palette::Palette;
use iced::{
    Element, Length, Point, Rectangle, Renderer, Theme, alignment, mouse,
    widget::{
        Space,
        canvas::{self, Canvas, Frame, Geometry, Path, Stroke},
        column, row, text,
    },
};

struct Graph {
    data: Vec<f32>,
    maximum_value: f32,
}

pub fn view<Message: 'static>(
    data: Vec<f32>,
    maximum_value: f32,
    height: f32,
) -> Element<'static, Message> {
    row![
        column![
            text("100%").size(12),
            Space::new().height(Length::Fill),
            text("50%").size(12),
            Space::new().height(Length::Fill),
            text("0%").size(12)
        ]
        .height(Length::Fixed(height))
        .align_x(alignment::Horizontal::Right),
        column![
            Canvas::new(Graph {
                data,
                maximum_value,
            })
            .width(Length::Fill)
            .height(Length::Fixed(height)),
            row![
                Space::new().width(Length::Fill),
                text("60 seconds").size(12),
                Space::new().width(Length::Fill),
            ]
        ]
        .spacing(8)
    ]
    .spacing(12)
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

            curve(builder, &points);

            builder.line_to(Point::new(width, height));
            builder.close();
        });

        let line = Path::new(|builder| {
            builder.move_to(points[0]);
            curve(builder, &points);
        });

        frame.fill(&filled_area, Palette::from(theme).accent_light);
        frame.stroke(
            &line,
            Stroke::default()
                .with_width(2.0)
                .with_color(Palette::from(theme).accent),
        );
        vec![frame.into_geometry()]
    }
}

fn curve(builder: &mut canvas::path::Builder, points: &[Point]) {
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        let mid = Point::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0);

        builder.quadratic_curve_to(p1, mid);
    }

    if let Some(&last) = points.last() {
        builder.line_to(last);
    }
}
