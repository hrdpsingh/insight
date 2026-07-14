use iced::{
    Element, Font, Theme,
    font::Weight,
    widget::{column, container, text},
};

use crate::{app::Message, palette::Palette};

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    container(column![
        text(label).size(12).wrapping(text::Wrapping::None),
        text(value)
            .style(move |theme: &Theme| text::Style {
                color: Some(Palette::from(theme).faded),
            })
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            }),
    ])
    .clip(true)
    .into()
}
