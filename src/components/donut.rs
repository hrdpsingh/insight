use iced::alignment::Vertical;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text};
use iced::{Color, Element, Pixels, Radians, Rectangle, Renderer, Theme, mouse};

struct DonutChart {
    used: u64,
    total: u64,
    first_arc_color: Color,
    second_arc_color: Color,
    thickness: f32,
}

pub fn view<'a, Message: 'a>(
    used: u64,
    total: u64,
    first_arc_color: Color,
    second_arc_color: Color,
    thickness: f32,
) -> Element<'a, Message, Theme, Renderer> {
    Canvas::new(DonutChart {
        used,
        total,
        first_arc_color,
        second_arc_color,
        thickness,
    })
    .width(140)
    .height(140)
    .into()
}

impl<Message> canvas::Program<Message, Theme, Renderer> for DonutChart {
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

        let center = frame.center();
        let outer_radius = bounds.width.min(bounds.height) / 2.0;
        let middle_radius = (outer_radius - self.thickness / 2.0).max(0.0);

        let percentage = self.used as f32 / self.total as f32;

        let start_angle = -std::f32::consts::FRAC_PI_2;
        let end_angle = start_angle + percentage * 2.0 * std::f32::consts::PI;

        let gap_pixels = self.thickness + 4.0;
        let gap_angle = gap_pixels / middle_radius;

        let filled_start = start_angle + gap_angle / 2.0;
        let filled_end = end_angle - gap_angle / 2.0;

        let bg_start = end_angle + gap_angle / 2.0;
        let bg_end = start_angle + 2.0 * std::f32::consts::PI - gap_angle / 2.0;

        if filled_end > filled_start {
            let filled_path = Path::new(|b| {
                b.arc(canvas::path::Arc {
                    center,
                    radius: middle_radius,
                    start_angle: Radians(filled_start),
                    end_angle: Radians(filled_end),
                });
            });
            frame.stroke(
                &filled_path,
                Stroke {
                    style: canvas::Style::Solid(self.first_arc_color),
                    width: self.thickness,
                    line_cap: canvas::LineCap::Round,
                    ..Stroke::default()
                },
            );
        }

        if bg_end > bg_start {
            let bg_path = Path::new(|b| {
                b.arc(canvas::path::Arc {
                    center,
                    radius: middle_radius,
                    start_angle: Radians(bg_start),
                    end_angle: Radians(bg_end),
                });
            });
            frame.stroke(
                &bg_path,
                Stroke {
                    style: canvas::Style::Solid(self.second_arc_color),
                    width: self.thickness,
                    line_cap: canvas::LineCap::Round,
                    ..Stroke::default()
                },
            );
        }

        let text = Text {
            content: format!("{:.1}%", percentage * 100.0),
            position: center,
            color: Color::BLACK,
            size: Pixels(20.0),
            align_x: iced::widget::text::Alignment::Center,
            align_y: Vertical::Center,
            ..Text::default()
        };
        frame.fill_text(text);

        vec![frame.into_geometry()]
    }
}
