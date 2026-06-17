use crate::app::Message;
use iced::{Background, Border, Color, Element, Length, Renderer, Theme, widget::container};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    background: Color,
) -> Element<'a, Message, Theme, Renderer> {
    container(content)
        .style(move |_| container::Style {
            background: Some(Background::Color(background)),
            border: Border {
                color: Color::from_rgb8(180, 185, 190),
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        })
        .padding(16)
        .width(Length::Fill)
        .into()
}