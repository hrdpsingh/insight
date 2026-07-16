use iced::{
    Element, Length, padding,
    widget::{Space, column, row},
};

use crate::{
    app::Message,
    components::{self, card, donut},
    metrics::format_bytes,
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
                    components::inline::view(
                        "Free",
                        format_bytes(insight.memory.total - insight.memory.used)
                    ),
                    components::inline::view("Used", format_bytes(insight.memory.used)),
                ]
                .spacing(8),
                Space::new().width(Length::Fill),
                donut::view(
                    insight.memory.used,
                    insight.memory.total,
                    |palette| palette.accent,
                    |palette| palette.accent_light,
                    12.0,
                ),
            ],
            components::stacked::view("Total", format_bytes(insight.memory.total))
        ]
        .spacing(24),
        padding::all(20.0),
        Length::Fixed(340.0),
    )
}
