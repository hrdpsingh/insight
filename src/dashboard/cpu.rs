use crate::{
    app::Message,
    components::{card, graph, title},
    state::Probe,
};
use iced::{
    Color, Element, Font,
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
                Color::from_rgb8(215, 235, 255),
                Color::from_rgb8(55, 155, 255),
                Color::from_rgb8(175, 215, 255),
            ),
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
        .spacing(12),
        iced::Length::Fill,
    )
}
