use crate::{
    app::Message,
    components::{self, card},
    metrics::format_bytes,
    state::Insight,
};
use iced::{
    Element, Length, alignment, padding,
    widget::{Space, column, row, text, tooltip},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    card::view(
        column![
            row![
                components::title::view("Storage".to_string()),
                Space::new().width(Length::Fill),
                components::tooltip::view(
                    components::button::view(
                        components::svg::view(include_bytes!("../../icons/refresh.svg").as_ref()),
                        Option::Some(Message::Refresh),
                        false
                    ),
                    components::stacked::view("Last Refreshed", insight.storage.time.clone()),
                    tooltip::Position::Top
                )
            ]
            .align_y(alignment::Vertical::Center),
            column![
                row![
                    text("Usage"),
                    Space::new().width(Length::Fill),
                    text(format!(
                        "{:.1}%",
                        (insight.storage.used * 100) as f32 / insight.storage.total as f32
                    ))
                ],
                components::bar::view(
                    insight.storage.used,
                    insight.storage.total,
                    |palette| palette.accent,
                    |palette| palette.accent_light,
                    12.0,
                ),
            ]
            .spacing(8)
            .width(Length::Fill),
            row![
                components::stacked::view("Used", format_bytes(insight.storage.used)),
                Space::new().width(Length::Fill),
                components::stacked::view("Total", format_bytes(insight.storage.total)),
            ]
            .spacing(4),
        ]
        .spacing(24),
        Length::Fixed(340.0),
        |palette| palette.surface,
        padding::all(20.0),
    )
}
