use crate::app::Message;
use iced::{
    Color, Element, Renderer, Theme, Length, border,
    widget::{Row, container},
};

pub fn view<'a>(content: Row<'a, Message, Theme, Renderer>) -> Element<'a, Message> {
    container(content)
        .style(|theme| {
            let mut style = container::transparent(theme);
            style.background = Some(Color::from_rgb8(255, 255, 255).into());
            style.border.radius = border::radius(5.0);
            style
        })
        .width(Length::Fill)
        .padding(20)
        .into()
}