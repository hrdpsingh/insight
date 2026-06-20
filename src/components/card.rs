use crate::app::Message;
use iced::{Background, Border, Color, Element, Length, Renderer, Theme, widget::container};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    width: Length,
) -> Element<'a, Message, Theme, Renderer> {
    container(content)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgb8(250, 250, 250))),
            border: Border {
                color: Color::from_rgb8(200, 200, 200),
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        })
        .padding(20)
        .width(width)
        .into()
}
