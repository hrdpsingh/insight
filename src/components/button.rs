use crate::app::Message;
use iced::{Border, Color, Element, border::Radius, padding, widget::button};

pub fn view(content: &str, navigate: Option<Message>) -> Element<'_, Message> {
    match navigate {
        Some(message) => button(content)
            .on_press(message)
            .padding(padding::horizontal(8).vertical(4))
            .style(move |_, _| button::Style {
                text_color: Color::from_rgb8(255, 255, 255),
                background: Some(Color::from_rgb8(150, 150, 255).into()),
                border: Border {
                    radius: Radius::from(4.0),
                    ..Default::default()
                },
                ..button::Style::default()
            }),
        None => button(content)
            .padding(padding::horizontal(8).vertical(4))
            .style(move |_, _| button::Style {
                text_color: Color::from_rgb8(225, 225, 255),
                background: Some(Color::from_rgb8(180, 180, 255).into()),
                border: Border {
                    radius: Radius::from(4.0),
                    ..Default::default()
                },
                ..button::Style::default()
            }),
    }
    .into()
}
