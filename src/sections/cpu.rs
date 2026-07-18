use crate::{
    app::Message,
    components::{self, card, graph},
    state::Insight,
};
use iced::{
    Element, Length, padding,
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
            graph::view(insight.cpu.history.clone(), 100.0, 100.0,),
            column![
                components::stacked::view("Name", insight.cpu.name.trim().to_string()),
                components::stacked::view("Logical Cores", insight.cpu.core_count.to_string()),
                components::stacked::view("Architecture", insight.cpu.architecture.clone()),
            ]
            .spacing(8),
        ]
        .spacing(20),
        padding::all(20.0),
        Length::Fixed(340.0),
    )
}
