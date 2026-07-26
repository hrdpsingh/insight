use crate::{app::Message, palette::Palette};
use iced::{Border, Element, Length, Padding, Renderer, Theme, widget::container};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    padding: Padding,
    width: Length,
    height: Length,
) -> Element<'a, Message, Theme, Renderer> {
    container(content)
        .style(move |theme| {
            container::Style::default()
                .background(Palette::from(theme).surface)
                .border(
                    Border::default()
                        .rounded(12.0)
                        .width(1)
                        .color(Palette::from(theme).border),
                )
        })
        .padding(padding)
        .height(height)
        .width(width)
        .into()
}
