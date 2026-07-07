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
                    Space::new().height(Length::Fill),
                    components::stacked::view("Total", format_bytes(insight.memory.total)),
                ]
                .spacing(8),
                donut::view(
                    insight.memory.used,
                    insight.memory.total,
                    insight.palette().accent,
                    insight.palette().accent_light,
                    12.0,
                ),
            ]
            .spacing(28),
        ]
        .spacing(24),
        Length::Shrink,
        |palette| palette.surface,
        padding::all(20.0),
    )
}
