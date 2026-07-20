use crate::{app::Message, palette::Palette};
use iced::{Background, Border, Element, Length, Padding, Renderer, Theme, widget::container};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    padding: Padding,
    width: Length,
    height: Length,
    surface: bool,
) -> Element<'a, Message, Theme, Renderer> {
    container(content)
        .style(move |theme| container::Style {
            background: if surface {
                Some(Background::Color(Palette::from(theme).surface))
            } else {
                Some(Background::Color(Palette::from(theme).background))
            },
            border: Border::default().rounded(8.0),
            ..container::Style::default()
        })
        .padding(padding)
        .height(height)
        .width(width)
        .into()
}
