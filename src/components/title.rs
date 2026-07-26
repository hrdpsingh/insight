use crate::{app::Message, palette::Palette};
use iced::{Element, Font, Theme, font::Weight::Bold, widget::text};

pub fn view<'a>(label: String) -> Element<'a, Message> {
    text(label)
        .size(24)
        .wrapping(text::Wrapping::None)
        .font(Font {
            weight: Bold,
            ..Font::default()
        })
        .style(move |theme: &Theme| text::Style {
            color: Some(Palette::from(theme).muted),
        })
        .into()
}
