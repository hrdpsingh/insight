use iced::{
    Color, Element, Font,
    font::Weight,
    widget::{column, text},
};

use crate::app::Message;

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    column![
        text(label)
            .color(Color::from_rgb8(100, 100, 100))
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            }),
        text(value),
    ]
    .into()
}
