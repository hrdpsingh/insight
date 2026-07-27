use iced::{
    Element, Font, Theme,
    font::Weight,
    widget::{column, container, text},
};

use crate::{app::Message, palette::Palette};

pub fn view(label: &str, value: String) -> Element<'_, Message> {
    container(column![
        text(label)
            .size(12)
            .wrapping(text::Wrapping::None)
            .style(move |theme: &Theme| text::Style {
                color: Some(Palette::from(theme).muted),
            }),
        text(value).font(Font {
            weight: Weight::Bold,
            ..Font::DEFAULT
        }),
    ])
    .clip(true)
    .into()
}
