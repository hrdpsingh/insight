use iced::{
    Border, Element, Length, Renderer, Theme,
    widget::{container, tooltip},
};

use crate::{app::Message, constant, palette::Palette};

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
                    .background(Palette::from(theme).surface)
                    .border(
                        Border::default()
                            .rounded(constant::border::RADIUS)
                            .width(constant::border::WIDTH)
                            .color(Palette::from(theme).border),
                    )
            })
            .padding(constant::padding::SMALL)
            .height(Length::Shrink)
            .width(Length::Shrink),
        position,
    )
    .into()
}
