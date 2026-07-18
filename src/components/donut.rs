use iced::{
    Element, Pixels, Radians, Rectangle, Renderer, Theme,
    alignment::Vertical,
    mouse,
    widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text},
};

use crate::palette::Palette;

struct DonutChart {
    used: u64,
    total: u64,
    thickness: f32,
}

pub fn view<'a, Message: 'a>(
    used: u64,
    total: u64,
    thickness: f32,
) -> Element<'a, Message, Theme, Renderer> {
    Canvas::new(DonutChart {
        used,
        total,
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
        theme: &Theme,
        bounds: Rectangle,
        _: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let center = frame.center();
        let outer_radius = bounds.width.min(bounds.height) / 2.0;
        let middle_radius = (outer_radius - self.thickness / 2.0).max(0.0);

        let percentage = if self.total == 0 {
            0.0
        } else {
            (self.used as f32 / self.total as f32).clamp(0.0, 1.0)
        };

        let start_angle = -std::f32::consts::FRAC_PI_2;
        let end_angle = start_angle + percentage * std::f32::consts::TAU;

        let background = Path::new(|builder| {
            builder.arc(canvas::path::Arc {
                center,
                radius: middle_radius,
                start_angle: Radians(0.0),
                end_angle: Radians(std::f32::consts::TAU),
            });
        });

        frame.stroke(
            &background,
            Stroke {
                style: canvas::Style::Solid(Palette::from(theme).accent_light),
                width: self.thickness,
                line_cap: canvas::LineCap::Round,
                ..Stroke::default()
            },
        );

        if percentage > 0.0 {
            let progress = Path::new(|builder| {
                builder.arc(canvas::path::Arc {
                    center,
                    radius: middle_radius,
                    start_angle: Radians(start_angle),
                    end_angle: Radians(end_angle),
                });
            });

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

        let text = Text {
            content: format!("{:.1}%", percentage * 100.0),
            position: center,
            color: Palette::from(theme).text,
            size: Pixels(20.0),
            align_x: iced::widget::text::Alignment::Center,
            align_y: Vertical::Center,
            ..Text::default()
        };
        frame.fill_text(text);

        vec![frame.into_geometry()]
    }
}
