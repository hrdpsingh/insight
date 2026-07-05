use iced::{
    Element,
    widget::{row, text},
};

use crate::{app::Message, palette};

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    row![text(label), text(": "), text(value).color(palette::ACCENT)].into()
}
