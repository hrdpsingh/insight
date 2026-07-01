use crate::{
    app::Message,
    components::{self, card, graph},
    state::Insight,
};
use iced::{
    Color, Element, Length, padding,
    widget::{Space, column, row},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    card::view(
        column![
            row![
                components::title::view("CPU".to_string()),
                Space::new().width(Length::Fill),
                components::title::view(format!(
                    "{:.1}%",
                    insight.cpu.history.last().copied().unwrap_or(0.0)
                )),
            ],
            graph::view(
                insight.cpu.history.clone(),
                100.0,
                150.0,
                Color::from_rgb8(220, 220, 255),
                Color::from_rgb8(150, 150, 255),
            ),
            column![
                components::constant::view("Name", insight.cpu.name.trim().to_string()),
                components::constant::view("Logical Cores", insight.cpu.core_count.to_string()),
                components::constant::view("Architecture", insight.cpu.architecture.clone()),
            ]
            .spacing(8),
        ]
        .spacing(20),
        Length::Shrink,
        Color::from_rgb8(240, 240, 250),
        padding::all(20.0),
    )
}
