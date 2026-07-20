use iced::{
    Background, Border, Element, Length, Renderer, Theme,
    widget::{container, tooltip},
};

use crate::{app::Message, palette::Palette};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    tooltip_content: impl Into<Element<'a, Message, Theme, Renderer>>,
    position: tooltip::Position,
) -> Element<'a, Message, Theme, Renderer> {
    tooltip(
        content,
        container(tooltip_content)
            .style(move |theme| container::Style {
                background: Some(Background::Color(Palette::from(theme).elevated)),
                border: Border::default().rounded(8.0),
                ..container::Style::default()
            })
            .padding(8)
            .height(Length::Shrink)
            .width(Length::Shrink),
        position,
    )
    .into()
}
