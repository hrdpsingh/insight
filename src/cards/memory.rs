use iced::{
    Border, Color, Element, Font, Length, Shadow, Vector,
    alignment::Vertical,
    font::Weight,
    padding,
    widget::{Space, column, container, row, text},
};

use crate::{
    app::Message,
    components::{card, donut},
    state::Insight,
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    card::view(
        column![
            row![
                Space::new().width(Length::Fill),
                container(
                    text("Memory")
                        .size(24)
                        .font(Font {
                            weight: Weight::Bold,
                            ..Font::DEFAULT
                        })
                        .color(Color::from_rgb8(100, 100, 100))
                )
                .padding(padding::top(4).bottom(4).left(16).right(16))
                .style(move |_| container::Style {
                    background: Some(Color::from_rgb8(245, 245, 255).into()),
                    border: Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    shadow: Shadow {
                        color: Color::from_rgb8(190, 190, 190),
                        offset: Vector::new(1.0, 1.0),
                        blur_radius: 4.0,
                    },
                    ..container::Style::default()
                }),
                Space::new().width(Length::Fill),
            ],
            row![
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
                            (insight.memory.total - insight.memory.used) as f32
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
                            insight.memory.used as f32 / (1024 * 1024 * 1024) as f32,
                        ))
                        .size(16)
                    ]
                    .spacing(8)
                    .align_y(Vertical::Center),
                    Space::new().height(Length::Fill),
                    column![
                        text("Total")
                            .size(16)
                            .color(Color::from_rgb8(100, 100, 100))
                            .font(Font {
                                weight: Weight::Bold,
                                ..Font::DEFAULT
                            }),
                        text(format!(
                            "{:.1} GB",
                            insight.memory.total as f32 / (1024 * 1024 * 1024) as f32,
                        ))
                        .size(16),
                    ],
                ]
                .spacing(8),
                donut::view(
                    insight.memory.used,
                    insight.memory.total,
                    Color::from_rgb8(150, 150, 255),
                    Color::from_rgb8(200, 200, 255),
                    12.0,
                ),
            ]
            .spacing(28),
        ]
        .spacing(24),
        Length::Shrink,
    )
}
