use crate::{
    app::Message,
    components::{card, graph},
    state::Insight,
};
use iced::{
    Border, Color, Element, Font, Length, Shadow, Vector,
    font::Weight,
    padding,
    widget::{Space, column, container, row, text},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    card::view(
        column![
            row![
                container(
                    text("CPU")
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
                container(
                    text(format!(
                        "{:.1}%",
                        insight.cpu.history.last().copied().unwrap_or(0.0)
                    ))
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
            ],
            graph::view(
                insight.cpu.history.clone(),
                100.0,
                150.0,
                Color::from_rgb8(220, 220, 255),
                Color::from_rgb8(130, 130, 255),
            ),
            column![
                column![
                    text("Name")
                        .size(16)
                        .color(Color::from_rgb8(100, 100, 100))
                        .font(Font {
                            weight: Weight::Bold,
                            ..Font::DEFAULT
                        }),
                    text(insight.cpu.name.trim()).size(16),
                ],
                column![
                    text("Logical Cores")
                        .size(16)
                        .color(Color::from_rgb8(100, 100, 100))
                        .font(Font {
                            weight: Weight::Bold,
                            ..Font::DEFAULT
                        }),
                    text(insight.cpu.core_count.to_string()).size(16),
                ],
                column![
                    text("Architecture")
                        .size(16)
                        .color(Color::from_rgb8(100, 100, 100))
                        .font(Font {
                            weight: Weight::Bold,
                            ..Font::DEFAULT
                        }),
                    text(insight.cpu.architecture.clone()).size(16),
                ],
            ]
            .spacing(8),
        ]
        .spacing(20),
        Length::Shrink,
    )
}
