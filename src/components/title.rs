use crate::{app::Message, components::card, palette::Palette};
use iced::{Element, Font, Length, Theme, font::Weight, padding, widget::text};

pub fn view<'a>(label: String) -> Element<'a, Message> {
    card::view(
        text(label)
            .size(20)
            .wrapping(text::Wrapping::None)
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            })
            .style(move |theme: &Theme| text::Style {
                color: Some(Palette::from(theme).faded),
            }),
        Length::Shrink,
        |palette| palette.elevated,
        padding::horizontal(16).vertical(8),
    )
}
