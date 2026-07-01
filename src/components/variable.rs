use iced::{
    Color, Element, Font,
    widget::{row, text},
};

use crate::app::Message;

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    row![
        text(label).font(Font { ..Font::MONOSPACE }),
        text(": ").font(Font { ..Font::MONOSPACE }),
        text(value)
            .color(Color::from_rgb8(140, 140, 255))
            .font(Font { ..Font::MONOSPACE }),
    ]
    .into()
}
