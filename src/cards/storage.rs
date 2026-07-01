use crate::{
    app::Message,
    components::{self, card},
    state::Insight,
};
use iced::{
    Border, Color, Element, Font, Length, Shadow, Vector,
    font::Weight,
    padding,
    widget::{Space, column, container, row, text},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    let total_space: u64 = insight.storage.disks.iter().map(|disk| disk.total).sum();
    let available_space: u64 = insight
        .storage
        .disks
        .iter()
        .map(|disk| disk.available)
        .sum();
    let used_space = total_space - available_space;

    card::view(
        column![
            row![
                Space::new().width(Length::Fill),
                container(
                    text("Storage")
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
            column![
                row![
                    text("Usage").size(16),
                    Space::new().width(Length::Fill),
                    text(format!("{}%", (used_space * 100) / total_space))
                ],
                components::bar::view(
                    used_space,
                    total_space,
                    Color::from_rgb8(150, 150, 255),
                    Color::from_rgb8(200, 200, 255),
                    12.0,
                ),
            ]
            .spacing(8)
            .width(Length::Fixed(240.0)),
            column![
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
                        total_space as f32 / (1024 * 1024 * 1024) as f32,
                    ))
                    .size(16),
                ],
                column![
                    text("Used")
                        .size(16)
                        .color(Color::from_rgb8(100, 100, 100))
                        .font(Font {
                            weight: Weight::Bold,
                            ..Font::DEFAULT
                        }),
                    text(format!(
                        "{:.1} GB",
                        used_space as f32 / (1024 * 1024 * 1024) as f32,
                    ))
                    .size(16),
                ]
            ]
            .spacing(8),
        ]
        .spacing(24),
        Length::Shrink,
    )
}
