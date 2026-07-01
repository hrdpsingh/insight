use iced::{Color, Element, Font, Length, font::Weight, padding, widget::text};

use crate::{app::Message, components::card};

pub fn view<'a>(label: String) -> Element<'a, Message> {
    card::view(
        text(label)
            .size(24)
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            })
            .color(Color::from_rgb8(100, 100, 100)),
        Length::Shrink,
        Color::from_rgb8(245, 245, 255),
        padding::horizontal(16).vertical(4),
    )
}
