use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Theme, mouse};

struct ProgressBar {
    used: f32,
    total: f32,
    first_color: Color,
    second_color: Color,
    thickness: f32,
}

pub fn view<'a, Message: 'a>(
    used: f32,
    total: f32,
    first_color: Color,
    second_color: Color,
    thickness: f32,
) -> Element<'a, Message, Theme, Renderer> {
    Canvas::new(ProgressBar {
        used,
        total,
        first_color,
        second_color,
        thickness,
    })
    .width(Length::Fill)
    .height(thickness)
    .into()
}

impl<Message> canvas::Program<Message, Theme, Renderer> for ProgressBar {
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
        let y = bounds.height / 2.0;

        let start_x = self.thickness / 2.0;
        let end_x = bounds.width - (self.thickness / 2.0);
        let usable_width = (end_x - start_x).max(0.0);

        let percentage = (self.used as f32 / self.total as f32).clamp(0.0, 1.0);
        let split_x = start_x + percentage * usable_width;

        let gap_pixels = self.thickness + 4.0;
        let half_gap = gap_pixels / 2.0;

        let first_start = start_x;
        let first_end = split_x - half_gap;

        if first_end > first_start {
            let first_path = Path::new(|builder| {
                builder.move_to(Point::new(first_start, y));
                builder.line_to(Point::new(first_end, y));
            });
            frame.stroke(
                &first_path,
                Stroke {
                    style: canvas::Style::Solid(self.first_color),
                    width: self.thickness,
                    line_cap: canvas::LineCap::Round,
                    ..Stroke::default()
                },
            );
        }

        let second_start = split_x + half_gap;
        let second_end = end_x;

        if second_end > second_start {
            let second_path = Path::new(|builder| {
                builder.move_to(Point::new(second_start, y));
                builder.line_to(Point::new(second_end, y));
            });
            frame.stroke(
                &second_path,
                Stroke {
                    style: canvas::Style::Solid(self.second_color),
                    width: self.thickness,
                    line_cap: canvas::LineCap::Round,
                    ..Stroke::default()
                },
            );
        }

        vec![frame.into_geometry()]
    }
}
