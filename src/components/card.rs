use crate::{app::Message, palette::Palette, state::ExtendTheme};
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
    container(content)
        .style(move |theme| {
            let multiplier = match theme {
                Theme::Dark => 1.15,
                _ => 0.92,
            };

            container::Style {
                background: Some(Background::Color(card_color(theme.custom()))),
                border: Border {
                    radius: 8.0.into(),
                    width: 2.0,
                    color: Color {
                        r: card_color(theme.custom()).r * multiplier,
                        g: card_color(theme.custom()).g * multiplier,
                        b: card_color(theme.custom()).b * multiplier,
                        a: card_color(theme.custom()).a,
                    },
                },
                shadow: Shadow {
                    color: theme.custom().shadow,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
                ..container::Style::default()
            }
        })
        .padding(padding)
        .width(width)
        .height(Length::Shrink)
        .clip(true)
        .into()
}
