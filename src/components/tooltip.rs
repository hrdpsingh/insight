use iced::{
    Border, Element, Length, Renderer, Theme,
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
            .style(move |theme| {
                container::Style::default()
                    .background(Palette::from(theme).elevated)
                    .border(Border::default().rounded(8.0))
            })
            .padding(8)
            .height(Length::Shrink)
            .width(Length::Shrink),
        position,
    )
    .into()
}
