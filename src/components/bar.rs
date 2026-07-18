use crate::palette::Palette;
use iced::{
    Element, Length, Point, Rectangle, Renderer, Theme, mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke},
};

struct ProgressBar {
    used: u64,
    total: u64,
    thickness: f32,
}

pub fn view<'a, Message: 'a>(
    used: u64,
    total: u64,
    thickness: f32,
) -> Element<'a, Message, Theme, Renderer> {
    Canvas::new(ProgressBar {
        used,
        total,
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
        theme: &Theme,
        bounds: Rectangle,
        _: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let y = bounds.height / 2.0;

        let start_x = self.thickness / 2.0;
        let end_x = bounds.width - self.thickness / 2.0;
        let usable_width = (end_x - start_x).max(0.0);

        let percentage = (self.used as f32 / self.total as f32).clamp(0.0, 1.0);
        let fill_end = start_x + percentage * usable_width;

        let background = Path::line(Point::new(start_x, y), Point::new(end_x, y));

        frame.stroke(
            &background,
            Stroke {
                style: canvas::Style::Solid(Palette::from(theme).accent_light),
                width: self.thickness,
                line_cap: canvas::LineCap::Round,
                ..Stroke::default()
            },
        );

        if fill_end > start_x {
            let progress = Path::line(Point::new(start_x, y), Point::new(fill_end, y));

            frame.stroke(
                &progress,
                Stroke {
                    style: canvas::Style::Solid(Palette::from(theme).accent),
                    width: self.thickness,
                    line_cap: canvas::LineCap::Round,
                    ..Stroke::default()
                },
            );
        }

        vec![frame.into_geometry()]
    }
}
