use iced::{
    Color, Element, Length, Rectangle, Renderer, Theme, mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke},
};

struct Graph {
    data: Vec<f32>,
    maximum_value: f32,
    background_color: Color,
    line_color: Color,
    color_below_line: Color,
}

pub fn view<Message: 'static>(
    data: Vec<f32>,
    maximum_value: f32,
    height: f32,
    background_color: Color,
    line_color: Color,
    color_below_line: Color,
) -> Element<'static, Message> {
    Canvas::new(Graph {
        data,
        maximum_value,
        background_color,
        line_color,
        color_below_line,
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
        _: &Theme,
        bounds: Rectangle,
        _: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill_rectangle(iced::Point::ORIGIN, bounds.size(), self.background_color);

        let width = bounds.width;
        let height = bounds.height;
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

        let line = Path::new(|builder| {
            builder.move_to(points[0]);
            points[1..].iter().for_each(|&point| builder.line_to(point));
        });

        let filled_area = Path::new(|builder| {
            builder.move_to(iced::Point::new(0.0, height));
            points.iter().for_each(|&point| builder.line_to(point));
            builder.line_to(iced::Point::new(width, height));
        });

        frame.fill(&filled_area, self.color_below_line);
        frame.stroke(
            &line,
            Stroke {
                style: canvas::Style::Solid(self.line_color),
                width: 2.0,
                ..Stroke::default()
            },
        );

        vec![frame.into_geometry()]
    }
}
