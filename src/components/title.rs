use iced::{Element, Font, Length, font::Weight, padding, widget::text};

use crate::{app::Message, components::card, palette};

pub fn view<'a>(label: String) -> Element<'a, Message> {
    card::view(
        text(label)
            .size(24)
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            })
            .color(palette::MUTED),
        Length::Shrink,
        palette::ELEVATED,
        padding::horizontal(16).vertical(4),
    )
}
