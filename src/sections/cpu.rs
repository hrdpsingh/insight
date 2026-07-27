use crate::{
    app::Message, components::{self, card, graph}, constant, state::Insight,
};
use iced::{
    Element, Length,
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
                    insight.cpu.history.back().copied().unwrap_or(0.0)
                )),
            ],
            graph::view(insight.cpu.history.clone(), 100.0, 100.0,),
            column![
                components::stacked::view("Name", insight.cpu.name.trim().to_string()),
                row![
                    components::stacked::view("Logical Cores", insight.cpu.core_count.to_string()),
                    Space::new().width(Length::Fill),
                    components::stacked::view("Architecture", insight.cpu.architecture.clone()),
                ]
            ]
            .spacing(constant::SPACE_SMALL),
        ]
        .spacing(constant::SPACE_MEDIUM),
        Length::Fixed(360.0),
    )
}
