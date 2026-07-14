use crate::{app::Message, components, palette::Palette};
use iced::{
    Background, Border, Color, Element, Length, Padding, Renderer, Shadow, Theme, Vector,
    widget::container,
};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    width: Length,
    card_color: fn(&Palette) -> Color,
    padding: Padding,
) -> Element<'a, Message, Theme, Renderer> {
    container(
        container(
            container(content)
                .style(move |theme| container::Style {
                    background: Some(Background::Gradient(components::gradient::view(
                        card_color(Palette::from(theme)),
                        0.03,
                    ))),
                    border: Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    ..container::Style::default()
                })
                .padding(padding)
                .width(Length::Fill)
                .height(Length::Shrink),
        )
        .style(move |theme| container::Style {
            background: Some(Background::Gradient(components::gradient::view(
                card_color(Palette::from(theme)),
                0.1,
            ))),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..container::Style::default()
        })
        .padding(2),
    )
    .style(move |theme| container::Style {
        shadow: Shadow {
            color: Palette::from(theme).shadow,
            offset: Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        border: Border {
            radius: 8.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    })
    .width(width)
    .height(Length::Shrink)
    .clip(true)
    .into()
}
