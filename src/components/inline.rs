use iced::{
    Element, Length,
    widget::{Space, row, text},
};

use crate::{app::Message, palette::Palette};

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    row![
        text(label.to_string())
            .wrapping(text::Wrapping::None)
            .style(|theme| text::Style {
                color: Some(Palette::from(theme).muted)
            }),
        Space::new().width(Length::Fill),
        text(value).wrapping(text::Wrapping::None)
    ]
    .into()
}
