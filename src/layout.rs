use crate::{app::Message, sections, state::Insight};
use iced::{
    Element, Size,
    widget::{column, row},
};

pub fn view<'a>(insight: &'a Insight, size: Size) -> Element<'a, Message> {
    if size.width < 720.0 {
        column![
            sections::memory::view(insight),
            sections::cpu::view(insight),
            sections::storage::view(insight),
            sections::processes::view(insight),
            sections::network::view(insight),
        ]
        .spacing(24)
        .into()
    } else if size.width < 1140.0 {
        row![
            column![
                sections::network::view(insight),
                sections::memory::view(insight),
                sections::storage::view(insight),
            ]
            .spacing(24),
            column![
                sections::cpu::view(insight),
                sections::processes::view(insight),
            ]
            .spacing(24),
        ]
        .spacing(24)
        .into()
    } else {
        row![
            column![
                sections::network::view(insight),
                sections::memory::view(insight),
            ]
            .spacing(24),
            column![
                sections::cpu::view(insight),
                sections::storage::view(insight),
            ]
            .spacing(24),
            sections::processes::view(insight),
        ]
        .spacing(24)
        .into()
    }
}
