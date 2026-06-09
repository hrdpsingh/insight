use crate::app::Message;
use iced::{
    Background, Border, Color, Element, Length, Renderer, Theme,
    widget::{Column, column, container},
};

pub fn view(body: Column<Message, Theme, Renderer>) -> Element<Message> {
    column![
        container(body)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
                border: Border {
                    color: Color::from_rgb8(205, 210, 215),
                    width: 1.0,
                    radius: 5.0.into(),
                },
                ..container::Style::default()
            })
            .width(Length::Fill)
            .padding(20),
    ]
    .spacing(20)
    .into()
}
