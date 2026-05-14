use crate::{
    app::Message,
    components::{card, heading},
};
use iced::Element;
use iced::widget::{column, row, text};

pub fn view<'a>() -> Element<'a, Message> {
    let content = row![text("Version")];

    column![heading::view("Kernel"), card::view(content)].into()
}
