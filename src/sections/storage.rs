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
                components::bar::view(insight.storage.used, insight.storage.total, 12.0,),
            ]
            .spacing(8)
            .width(Length::Fill),
            column![
                components::stacked::view("Used", format_bytes(insight.storage.used)),
                components::stacked::view("Total", format_bytes(insight.storage.total)),
            ]
            .spacing(8),
        ]
        .spacing(24),
        padding::all(20.0),
        Length::Fixed(340.0),
    )
}
