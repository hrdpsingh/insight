use crate::app::Message;
use iced::{Color, Element, Font, font::Weight, widget::text};

pub fn view<'a>(label: &'a str) -> Element<'a, Message> {
    text(label)
        .size(20)
        .font(Font {
            weight: Weight::Bold,
            ..Font::MONOSPACE
        })
        .color(Color::from_rgb8(100, 100, 100))
        .into()
}
