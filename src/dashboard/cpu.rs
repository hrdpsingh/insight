use crate::{
    app::Message,
    components::{card, graph, title},
    state::Probe,
};
use iced::{
    Color, Element, Font, Length,
    font::Weight,
    widget::{column, text},
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    card::view(
        column![
            title::view(format!(
                "CPU {:.1}%",
                probe.cpu.history.last().copied().unwrap_or(0.0)
            )),
            graph::view(
                probe.cpu.history.clone(),
                100.0,
                150.0,
                Color::from_rgb8(220, 220, 255),
                Color::from_rgb8(120, 120, 255),
                Color::from_rgb8(180, 180, 255),
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
                    text(probe.cpu.name.trim()).size(16),
                ],
                column![
                    text("Cores")
                        .size(16)
                        .color(Color::from_rgb8(100, 100, 100))
                        .font(Font {
                            weight: Weight::Bold,
                            ..Font::DEFAULT
                        }),
                    text(probe.cpu.core_count.to_string()).size(16),
                ],
                column![
                    text("Architecture")
                        .size(16)
                        .color(Color::from_rgb8(100, 100, 100))
                        .font(Font {
                            weight: Weight::Bold,
                            ..Font::DEFAULT
                        }),
                    text(probe.cpu.architecture.clone()).size(16),
                ],
            ]
            .spacing(8),
        ]
        .spacing(20),
        Length::Shrink,
    )
}
