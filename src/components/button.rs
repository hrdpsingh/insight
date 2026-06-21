use crate::app::Message;
use iced::{Border, Color, Element, border::Radius, widget::button};

pub fn view(content: &str, navigate: Message) -> Element<'_, Message> {
    button(content)
        .on_press(navigate)
        .padding(8)
        .style(move |_, _| button::Style {
            text_color: Color::from_rgb8(55, 155, 255),
            border: Border {
                radius: Radius::from(8.0),
                ..Default::default()
            },
            ..button::Style::default()
        })
        .into()
}
