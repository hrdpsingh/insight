use iced::{
    Element, Length, padding,
    widget::{Space, column, row},
};

use crate::{
    app::Message,
    components::{self, card, donut},
    palette,
    state::Insight,
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    card::view(
        column![
            row![
                Space::new().width(Length::Fill),
                components::title::view("Memory".to_string()),
                Space::new().width(Length::Fill),
            ],
            row![
                column![
                    components::variable::view(
                        "Free",
                        format!("{:.1} GB", insight.memory.total - insight.memory.used,)
                    ),
                    components::variable::view("Used", format!("{:.1} GB ", insight.memory.used)),
                    Space::new().height(Length::Fill),
                    components::constant::view("Total", format!("{:.1} GB", insight.memory.total)),
                ]
                .spacing(8),
                donut::view(
                    insight.memory.used,
                    insight.memory.total,
                    palette::ACCENT,
                    palette::ACCENT_LIGHT,
                    12.0,
                ),
            ]
            .spacing(28),
        ]
        .spacing(24),
        Length::Shrink,
        palette::CARD,
        padding::all(20.0),
    )
}
