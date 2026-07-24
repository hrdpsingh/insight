use crate::app::Message;
use iced::{Border, Color, Element, Length, Padding, Renderer, Theme, widget::container};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    padding: Padding,
    width: Length,
    height: Length,
    color: impl Fn(&Theme) -> Color + 'static,
) -> Element<'a, Message, Theme, Renderer> {
    container(content)
        .style(move |theme| {
            container::Style::default()
                .background(color(theme))
                .border(Border::default().rounded(8.0))
        })
        .padding(padding)
        .height(height)
        .width(width)
        .into()
}
