use crate::app::Message;
use iced::{
    Border, Color, Element, Length, Padding, Renderer, Shadow, Theme, Vector, widget::container,
};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    width: Length,
    color: Color,
    padding: Padding,
) -> Element<'a, Message, Theme, Renderer> {
    container(content)
        .style(move |_| container::Style {
            background: Some(color.into()),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::from_rgb8(190, 190, 190),
                offset: Vector::new(1.0, 1.0),
                blur_radius: 4.0,
            },
            ..container::Style::default()
        })
        .padding(padding)
        .width(width)
        .height(Length::Shrink)
        .clip(true)
        .into()
}
