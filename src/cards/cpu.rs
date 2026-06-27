use crate::{
    app::Message,
    components::{card, graph, title},
    state::Insight,
};
use iced::{
    Color, Element, Font, Length,
    font::Weight,
    widget::{column, text},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    card::view(
        column![
            title::view(format!(
                "CPU {:.1}%",
                insight.cpu.history.last().copied().unwrap_or(0.0)
            )),
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
                    text("Cores")
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
