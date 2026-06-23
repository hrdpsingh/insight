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
            title::view("Swap".to_string()),
            donut::view(
                probe.swap.used,
                probe.swap.total,
                Color::from_rgb8(55, 155, 255),
                Color::from_rgb8(175, 215, 255),
                16.0,
            ),
            column![
                row![
                    container(
                        Space::new()
                            .width(Length::Fixed(12.0))
                            .height(Length::Fixed(12.0))
                    )
                    .style(move |_| container::Style {
                        background: Some(Color::from_rgb8(175, 215, 255).into()),
                        border: Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..container::Style::default()
                    }),
                    text(format!(
                        "Free: {:.1} GB",
                        (probe.swap.total - probe.swap.used) as f32 / (1024 * 1024 * 1024) as f32,
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
                        background: Some(Color::from_rgb8(55, 155, 255).into()),
                        border: Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..container::Style::default()
                    }),
                    text(format!(
                        "Used: {:.1} GB",
                        probe.swap.used as f32 / (1024 * 1024 * 1024) as f32,
                    ))
                    .size(16)
                ]
                .spacing(8)
                .align_y(Vertical::Center)
            ]
            .spacing(8)
        ]
        .spacing(20),
        Length::Fill,
    )
}
