use iced::{
    Element,
    widget::{container, text},
};

use crate::app::Message;

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    container(text(format!("{label}: {value}")).wrapping(text::Wrapping::None))
        .clip(true)
        .into()
}
