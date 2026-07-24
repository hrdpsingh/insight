use iced::{
    Element, Length, alignment, padding,
    widget::{Space, column, row},
};

use crate::{
    app::Message,
    components::{self, card, donut},
    metrics::format_bytes,
    palette::Palette,
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
            column![
                column![donut::view(insight.memory.used, insight.memory.total, 12.0)]
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center),
                column![
                    components::inline::view(
                        "Free",
                        format_bytes(insight.memory.total - insight.memory.used),
                        200
                    ),
                    components::inline::view("Used", format_bytes(insight.memory.used), 200),
                    components::inline::view("Total", format_bytes(insight.memory.total), 200),
                ]
                .spacing(8)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
            ]
            .spacing(20),
        ]
        .spacing(24),
        padding::all(20.0),
        Length::Fixed(340.0),
        Length::Fixed(360.0),
        |theme| Palette::from(theme).surface,
    )
}
