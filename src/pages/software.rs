use crate::app::Message;
use iced::{Element, widget::{column, text}};

pub fn view<'a>() -> Element<'a, Message> {
    column![text("Software Page"),].into()
}

