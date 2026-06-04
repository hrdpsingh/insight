pub mod button;
use crate::{app::Message, components::card, models::Page};
use iced::{
    Element, widget::column,
};

pub fn view<'a>(page: Page) -> Element<'a, Message> {
    card::view(120, column![
        button::view("Software", Page::Software, page),
        button::view("CPU", Page::Cpu, page),
        button::view("Firmware", Page::Firmware, page),
    ].spacing(10))
}
