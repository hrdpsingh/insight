use iced::{
    Element, Font, Theme,
    font::Weight,
    widget::{column, container, text},
};

use crate::{app::Message, state::ExtendTheme};

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    container(column![
        text(label).size(12).wrapping(text::Wrapping::None),
        text(value)
            .wrapping(text::Wrapping::None)
            .style(move |theme: &Theme| text::Style {
                color: Some(theme.custom().faded),
            })
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            }),
    ])
    .clip(true)
    .into()
}
