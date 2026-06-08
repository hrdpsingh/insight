use crate::app::Message;
use iced::{Element, Font, font::Weight, widget::text};

pub fn view<'a>(content: &'a str) -> Element<'a, Message> {
    text(content)
        .size(24)
        .font(Font {
            weight: Weight::Bold,
            ..Font::default()
        })
        .into()
}
