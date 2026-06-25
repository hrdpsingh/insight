use crate::app::Message;
use iced::{Border, Color, Element, Length, Renderer, Shadow, Theme, Vector, widget::container};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    width: Length,
) -> Element<'a, Message, Theme, Renderer> {
    container(content)
        .style(move |_| container::Style {
            background: Some(Color::from_rgb8(250, 250, 250).into()),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::from_rgb8(200, 200, 200),
                offset: Vector::new(1.0, 1.0),
                blur_radius: 4.0,
            },
            ..container::Style::default()
        })
        .padding(20)
        .width(width)
        .into()
}
