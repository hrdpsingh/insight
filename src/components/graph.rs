use iced::{
    Color, Element, Length, Rectangle, Renderer, Theme, mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke},
};

pub struct Graph {
    height: f32,
    background_color: Color,
    line_color: Color,
    color_below_line: Color,
    data: Vec<f32>,
    maximum_value: f32,
}

impl Graph {
    pub fn new(
        height: f32,
        background_color: Color,
        line_color: Color,
        color_below_line: Color,
        data: Vec<f32>,
        maximum_value: f32,
    ) -> Self {
        Self {
            height,
            background_color,
            line_color,
            color_below_line,
            data,
            maximum_value,
        }
    }
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

        let gap = width / (self.data.len() - 1) as f32;

        let line = Path::new(|builder| {
            for (index, &value) in self.data.iter().enumerate() {
                let x = index as f32 * gap;
                let normalized_y = value / self.maximum_value;
                let y = height - (normalized_y * height);
                let point = iced::Point::new(x, y);

                if index == 0 {
                    builder.move_to(point);
                } else {
                    builder.line_to(point);
                }
            }
        });

        let filled_area = Path::new(|builder| {
            builder.move_to(iced::Point::new(0.0, height));

            for (index, &value) in self.data.iter().enumerate() {
                let x = index as f32 * gap;
                let normalized_y = value / self.maximum_value;
                let y = height - (normalized_y * height);
                builder.line_to(iced::Point::new(x, y));
            }

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

impl<'a, Message: 'a> From<Graph> for Element<'a, Message> {
    fn from(graph: Graph) -> Self {
        let height = graph.height;

        Canvas::new(graph)
            .width(Length::Fill)
            .height(Length::Fixed(height))
            .into()
    }
}
