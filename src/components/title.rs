use iced::{Element, Font, Length, Theme, font::Weight, padding, widget::text};

use crate::{app::Message, components::card, state::ExtendTheme};

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
                color: Some(theme.custom().faded),
            }),
        Length::Shrink,
        |palette| palette.elevated,
        padding::horizontal(16).vertical(8),
    )
}
