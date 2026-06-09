use crate::app::Message;
use crate::components::{card, graph::Graph};
use iced::{
    Color, Element, Font, font::Weight,
    widget::{column, row, text},
};

pub fn view<'a>(cpu_usage_history: Vec<f32>, cpu_name: &'a str, cpu_architecture: &'a str, core_count: usize) -> Element<'a, Message> {
    let cpu_usage = cpu_usage_history
        .last()
        .map(|u| format!("{:.2}%", u))
        .unwrap_or_else(|| "Unavailable".to_string());

    card::view(
        column![
            Graph::new(
                200.0,
                Color::from_rgb8(215, 235, 255),
                Color::from_rgb8(55, 155, 255),
                Color::from_rgb8(175, 215, 255),
                cpu_usage_history,
                100.0,
            ),
            row![
                text("Model: ").size(14).font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                }),
                text(cpu_name).size(14),
            ],
            row![
                text("Architecture: ").size(14).font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                }),
                text(cpu_architecture).size(14),
            ],
            row![
                text("Cores: ").size(14).font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                }),
                text(core_count).size(14),
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