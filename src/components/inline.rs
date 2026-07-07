use iced::{Element, widget::text};

use crate::app::Message;

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    text(format!("{label}: {value}")).into()
}
