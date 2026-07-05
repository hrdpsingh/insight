use iced::{
    Element, Font,
    font::Weight,
    widget::{column, text},
};

use crate::{app::Message, palette};

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    column![
        text(label).size(12),
        text(value).color(palette::MUTED).font(Font {
            weight: Weight::Bold,
            ..Font::DEFAULT
        }),
    ]
    .into()
}
