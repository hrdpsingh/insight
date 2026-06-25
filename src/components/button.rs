use crate::app::Message;
use iced::{Border, Color, Element, border::Radius, widget::button};

pub fn view(content: &str, navigate: Option<Message>) -> Element<'_, Message> {
    match navigate {
        Some(message) => button(content)
            .on_press(message)
            .padding(8)
            .style(move |_, _| button::Style {
                text_color: Color::from_rgb8(120, 120, 255),
                border: Border {
                    radius: Radius::from(8.0),
                    ..Default::default()
                },
                ..button::Style::default()
            }),
        None => button(content).padding(8).style(move |_, _| button::Style {
            text_color: Color::from_rgb8(180, 180, 255),
            border: Border {
                radius: Radius::from(8.0),
                ..Default::default()
            },
            ..button::Style::default()
        }),
    }
    .into()
}
