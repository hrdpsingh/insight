use iced::alignment::Vertical;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text};
use iced::{Color, Element, Length, Pixels, Radians, Rectangle, Renderer, Theme, mouse};

pub fn view<'a, Message: 'a>(
    used: u64,
    total: u64,
    filled_color: Color,
    bg_color: Color,
    thickness: f32, // Replaced inner/outer radius with thickness
) -> Element<'a, Message, Theme, Renderer> {
    Canvas::new(DonutChart {
        used,
        total,
        filled_color,
        bg_color,
        thickness,
    })
    .width(Length::Fill)  // Automatically spans the width of the parent
    .height(Length::Fill) // Automatically spans the height of the parent
    .into()
}

struct DonutChart {
    used: u64,
    total: u64,
    filled_color: Color,
    bg_color: Color,
    thickness: f32,
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

        // 1. Automatically determine max possible radius based on available bounds
        let max_radius = bounds.width.min(bounds.height) / 2.0;
        
        // 2. Prevent the stroke from clipping out of the bounding box by backing it off 
        // by half of the thickness. This becomes our drawing path.
        let middle_radius = (max_radius - self.thickness / 2.0).max(0.0);
        let inner_radius = (max_radius - self.thickness).max(0.0);

        let percentage = if self.total > 0 {
            (self.used as f32 / self.total as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Draw Background Ring
        let bg_path = Path::circle(center, middle_radius);
        frame.stroke(
            &bg_path,
            Stroke {
                style: canvas::Style::Solid(self.bg_color),
                width: self.thickness,
                ..Stroke::default()
            },
        );

        // Draw Progress Ring
        if percentage > 0.0 {
            let start_angle = -std::f32::consts::FRAC_PI_2;
            let end_angle = start_angle + percentage * 2.0 * std::f32::consts::PI;

            let progress_path = Path::new(|builder| {
                builder.arc(canvas::path::Arc {
                    center,
                    radius: middle_radius,
                    start_angle: Radians(start_angle),
                    end_angle: Radians(end_angle),
                });
            });

            frame.stroke(
                &progress_path,
                Stroke {
                    style: canvas::Style::Solid(self.filled_color),
                    width: self.thickness,
                    ..Stroke::default()
                },
            );
        }

        // Draw Percentage Text
        let text_content = format!("{:.1}%", percentage * 100.0);
        
        // Text dynamically scales using the computed inner hole radius
        let font_size = inner_radius * 0.45;

        if font_size > 0.0 {
            let text = Text {
                content: text_content,
                position: center,
                color: Color::BLACK,
                size: Pixels(font_size),
                align_x: iced::widget::text::Alignment::Center,
                align_y: Vertical::Center,
                ..Text::default()
            };
            frame.fill_text(text);
        }

        vec![frame.into_geometry()]
    }
}