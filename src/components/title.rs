use crate::app::Message;
use iced::{Element, Font, font::Weight, widget::text};

pub fn view<'a>() -> Element<'a, Message> {
    text("Usage")
        .size(24)
        .font(Font {
            weight: Weight::Bold,
            ..Font::default()
        })
        .into()
}
