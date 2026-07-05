use iced::{Element, Length, Renderer, Theme, padding, widget::tooltip};

use crate::{app::Message, components::card, palette};

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    tooltip_content: impl Into<Element<'a, Message, Theme, Renderer>>,
    position: tooltip::Position,
) -> Element<'a, Message, Theme, Renderer> {
    tooltip(
        content,
        card::view(
            tooltip_content,
            Length::Shrink,
            palette::ELEVATED,
            padding::all(8),
        ),
        position,
    )
    .into()
}
