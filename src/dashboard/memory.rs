use iced::{
    Border, Color, Element, Length,
    alignment::Vertical,
    widget::{Space, column, container, row, text},
};

use crate::{
    app::Message,
    components::{card, donut, title},
    state::Probe,
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    card::view(
        column![
            title::view(format!(
                "Memory {:.1} GB",
                probe.memory.total as f32 / (1024 * 1024 * 1024) as f32,
            )),
            row![
                Space::new().width(Length::Fill),
                donut::view(
                    probe.memory.used,
                    probe.memory.total,
                    Color::from_rgb8(150, 150, 255),
                    Color::from_rgb8(200, 200, 255),
                    12.0,
                ),
                Space::new().width(Length::Fill),
            ],
            column![
                row![
                    container(
                        Space::new()
                            .width(Length::Fixed(12.0))
                            .height(Length::Fixed(12.0))
                    )
                    .style(move |_| container::Style {
                        background: Some(Color::from_rgb8(200, 200, 255).into()),
                        border: Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..container::Style::default()
                    }),
                    text(format!(
                        "Free: {:.1} GB",
                        (probe.memory.total - probe.memory.used) as f32
                            / (1024 * 1024 * 1024) as f32,
                    ))
                    .size(16)
                ]
                .spacing(8)
                .align_y(Vertical::Center),
                row![
                    container(
                        Space::new()
                            .width(Length::Fixed(12.0))
                            .height(Length::Fixed(12.0))
                    )
                    .style(move |_| container::Style {
                        background: Some(Color::from_rgb8(150, 150, 255).into()),
                        border: Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..container::Style::default()
                    }),
                    text(format!(
                        "Used: {:.1} GB",
                        probe.memory.used as f32 / (1024 * 1024 * 1024) as f32,
                    ))
                    .size(16)
                ]
                .spacing(8)
                .align_y(Vertical::Center),
            ]
            .spacing(8),
        ]
        .spacing(24),
        Length::Shrink,
    )
}
