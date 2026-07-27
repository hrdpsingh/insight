use iced::{
    Element, Font, Length,
    font::Weight,
    widget::{Space, row, text},
};

use crate::{app::Message, palette::Palette};

pub fn view<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    row![
        text(String::from(label)).wrapping(text::Wrapping::None),
        Space::new().width(Length::Fill),
        text(value)
            .wrapping(text::Wrapping::None)
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            })
            .style(|theme| text::Style {
                color: Some(Palette::from(theme).muted)
            }),
    ]
    .into()
}
