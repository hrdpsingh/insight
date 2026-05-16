use iced::{
    Color, Element, Length, Point, Rectangle, Renderer, Size, Theme, mouse,
    widget::canvas::{self, Canvas, Geometry, Path, Program},
};

pub struct Graph {
    data: Vec<f32>,
    max_value: f32,
    line_color: Color,
    color_above_line: Color,
    color_below_line: Color,
}

impl Graph {
    pub fn new(
        data: Vec<f32>,
        max_value: f32,
        line_color: Color,
        color_above_line: Color,
        color_below_line: Color,
    ) -> Self {
        Self {
            data,
            max_value,
            line_color,
            color_above_line,
            color_below_line,
        }
    }
}

impl<Message> Program<Message, Theme, Renderer> for Graph {
    type State = canvas::Cache;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = state.draw(renderer, bounds.size(), |frame| {
            let Size { width, height } = bounds.size();
            let steps = (self.data.len() - 1) as f32;
            let x_step = width / steps;

            let get_point = |index: usize, val: f32| {
                let x = index as f32 * x_step;
                let normalized_y = (val / self.max_value).clamp(0.0, 1.0);
                let y = height * (1.0 - normalized_y);
                Point::new(x, y)
            };

            let above_path = Path::new(|p| {
                p.move_to(Point::new(0.0, 0.0));
                for (i, &val) in self.data.iter().enumerate() {
                    p.line_to(get_point(i, val));
                }
                p.line_to(Point::new(width, 0.0));
                p.close();
            });
            frame.fill(&above_path, self.color_above_line);

            let below_path = Path::new(|p| {
                p.move_to(Point::new(0.0, height));
                for (i, &val) in self.data.iter().enumerate() {
                    p.line_to(get_point(i, val));
                }
                p.line_to(Point::new(width, height));
                p.close();
            });
            frame.fill(&below_path, self.color_below_line);

            let line_path = Path::new(|p| {
                if let Some(&first) = self.data.first() {
                    p.move_to(get_point(0, first));
                    for (i, &val) in self.data.iter().enumerate().skip(1) {
                        p.line_to(get_point(i, val));
                    }
                }
            });

            let mut stroke = canvas::Stroke::default().with_color(self.line_color);
            stroke.width = 2.0;
            frame.stroke(&line_path, stroke);
        });

        vec![geometry]
    }
}

impl<'a, Message: 'a> From<Graph> for Element<'a, Message, Theme, Renderer> {
    fn from(graph: Graph) -> Self {
        Canvas::new(graph)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
