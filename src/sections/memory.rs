use iced::{
    Element, Length, alignment,
    widget::{column, rule},
};

use crate::{
    app::Message,
    components::{self, card, donut},
    constant,
    metrics::format_bytes,
    palette::Palette,
    state::Insight,
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    card::view(
        column![
            components::title::view("Memory".to_string()),
            column![
                column![donut::view(insight.memory.used, insight.memory.total, 12.0)]
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center),
                column![
                    components::inline::view(
                        "Free",
                        format_bytes(insight.memory.total - insight.memory.used)
                    ),
                    rule::horizontal(1).style(|theme| rule::Style {
                        color: Palette::from(theme).border,
                        radius: iced::border::Radius::new(0),
                        fill_mode: rule::FillMode::Full,
                        snap: false,
                    }),
                    components::inline::view("Used", format_bytes(insight.memory.used)),
                    rule::horizontal(1).style(|theme| rule::Style {
                        color: Palette::from(theme).border,
                        radius: iced::border::Radius::new(0),
                        fill_mode: rule::FillMode::Full,
                        snap: false,
                    }),
                    components::inline::view("Total", format_bytes(insight.memory.total)),
                ]
                .spacing(constant::spacing::SMALL)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
            ]
            .spacing(constant::spacing::MEDIUM),
        ]
        .spacing(constant::spacing::LARGE),
        Length::Fixed(constant::card::HEIGHT_MEDIUM),
    )
}
