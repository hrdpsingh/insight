use crate::app::Message;
use iced::{Element, Font, font::Weight, widget::text};

pub fn view(title: &str) -> Element<'_, Message> {
    text(title)
        .size(20)
        .font(Font {
            weight: Weight::Bold,
            ..Font::default()
        })
        .into()
}
