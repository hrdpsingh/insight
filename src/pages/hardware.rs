use crate::app::Message;
use iced::Element;
use iced::widget::{column, text};

pub fn view<'a>() -> Element<'a, Message> {
    column![text("Hardware Page"),].into()
}
