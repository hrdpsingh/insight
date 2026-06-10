use crate::{state::Probe, app::Message};
use crate::components::{card, graph::Graph};
use iced::{
    Color, Element, Font, font::Weight,
    widget::{column, row, text},
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    let cpu_usage = probe.processor.usage_history
        .last()
        .map(|usage| format!("{:.2}%", usage))
        .unwrap_or_else(|| "Unavailable".to_string());

    card::view(
        column![
            Graph::new(
                200.0,
                Color::from_rgb8(215, 235, 255),
                Color::from_rgb8(55, 155, 255),
                Color::from_rgb8(175, 215, 255),
                probe.processor.usage_history.clone(),
                100.0,
            ),
            row![
                text("Model: ").size(14).font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                }),
                text(&probe.processor.name).size(14),
            ],
            row![
                text("Architecture: ").size(14).font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                }),
                text(&probe.processor.architecture).size(14),
            ],
            row![
                text("Cores: ").size(14).font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                }),
                text(probe.processor.core_count).size(14),
            ],
            row![
                text("Usage: ").size(14).font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                }),
                text(cpu_usage).size(14),
            ],
        ]
        .spacing(10),
    )
}