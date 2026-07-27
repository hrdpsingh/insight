use crate::{app::Message, constant, palette::Palette};
use iced::{Border, Element, Length, Renderer, Theme, widget::container};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    height: Length,
) -> Element<'a, Message, Theme, Renderer> {
    container(content)
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
        .padding(constant::padding::LARGE)
        .width(constant::card::WIDTH)
        .height(height)
        .into()
}
