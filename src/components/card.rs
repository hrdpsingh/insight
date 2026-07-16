use crate::{app::Message, palette::Palette};
use iced::{Background, Border, Element, Length, Padding, Renderer, Theme, widget::container};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    padding: Padding,
    width: Length,
) -> Element<'a, Message, Theme, Renderer> {
    container(content)
        .style(move |theme| container::Style {
            background: Some(Background::Color(Palette::from(theme).surface)),
            border: Border::default()
                .rounded(8.0)
                .width(2.0)
                .color(Palette::from(theme).border),
            ..container::Style::default()
        })
        .padding(padding)
        .height(Length::Shrink)
        .width(width)
        .into()
}
