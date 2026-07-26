use iced::{
    Element, Length,
    widget::{Space, container, row, text},
};

use crate::{app::Message, palette::Palette};

pub fn view<'a>(label: &'a str, value: String, width: u32) -> Element<'a, Message> {
    container(row![
        text(label.to_string())
            .wrapping(text::Wrapping::None)
            .style(|theme| text::Style {
                color: Some(Palette::from(theme).muted)
            }),
        Space::new().width(Length::Fill),
        text(value).wrapping(text::Wrapping::None)
    ])
    .width(width)
    .into()
}
