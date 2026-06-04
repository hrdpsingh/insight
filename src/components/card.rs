use crate::app::Message;
use iced::{
    Color, Element, Length, Renderer, Shadow, Theme, Vector, border,
    widget::{Column, column, container},
};

pub fn view<'a>(
    width: impl Into<Length>,
    body: Column<'a, Message, Theme, Renderer>,
) -> Element<'a, Message> {
    column![
        container(body)
            .style(|theme| {
                let mut style = container::transparent(theme);
                style.background = Some(Color::from_rgb8(255, 255, 255).into());
                style.border.radius = border::radius(15.0);
                style.shadow = Shadow {
                    color: Color::from_rgb8(200, 200, 200),
                    offset: Vector::new(1.0, 1.0),
                    blur_radius: 5.0,
                };
                style
            })
            .width(width)
            .padding(20),
    ]
    .spacing(20)
    .into()
}
