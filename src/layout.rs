use crate::{app::Message, constant, sections, state::Insight};
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
            sections::network::view(insight),
            sections::processes::view(insight),
        ]
        .spacing(constant::SPACE_LARGE)
        .into()
    } else if size.width < 1140.0 {
        row![
            column![
                sections::cpu::view(insight),
                sections::storage::view(insight),
                sections::processes::view(insight),
            ]
            .spacing(constant::SPACE_LARGE),
            column![
                sections::network::view(insight),
                sections::memory::view(insight),
            ]
            .spacing(constant::SPACE_LARGE),
        ]
        .spacing(constant::SPACE_LARGE)
        .into()
    } else {
        row![
            column![
                sections::cpu::view(insight),
                sections::storage::view(insight),
            ]
            .spacing(constant::SPACE_LARGE),
            column![
                sections::network::view(insight),
                sections::memory::view(insight),
            ]
            .spacing(constant::SPACE_LARGE),
            sections::processes::view(insight),
        ]
        .spacing(constant::SPACE_LARGE)
        .into()
    }
}
