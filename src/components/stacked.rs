use iced::{
    Element, Font, Theme,
    font::Weight,
    widget::{column, text},
};

use crate::{app::Message, state::ExtendTheme};

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    column![
        text(label).size(12),
        text(value)
            .style(move |theme: &Theme| text::Style {
                color: Some(theme.custom().faded),
            })
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            }),
    ]
    .into()
}
