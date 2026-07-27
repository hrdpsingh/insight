use crate::{app::Message, constant, sections, state::Insight};
use iced::{
    Element, Size,
    widget::{column, row},
};

pub fn view(insight: &Insight, size: Size) -> Element<'_, Message> {
    if size.width < constant::breakpoint::NARROW {
        column![
            sections::memory::view(insight),
            sections::cpu::view(insight),
            sections::storage::view(insight),
            sections::network::view(insight),
            sections::processes::view(insight),
        ]
        .spacing(constant::spacing::LARGE)
        .into()
    } else if size.width < constant::breakpoint::WIDE {
        row![
            column![
                sections::cpu::view(insight),
                sections::storage::view(insight),
                sections::processes::view(insight),
            ]
            .spacing(constant::spacing::LARGE),
            column![
                sections::network::view(insight),
                sections::memory::view(insight),
            ]
            .spacing(constant::spacing::LARGE),
        ]
        .spacing(constant::spacing::LARGE)
        .into()
    } else {
        row![
            column![
                sections::cpu::view(insight),
                sections::storage::view(insight),
            ]
            .spacing(constant::spacing::LARGE),
            column![
                sections::network::view(insight),
                sections::memory::view(insight),
            ]
            .spacing(constant::spacing::LARGE),
            sections::processes::view(insight),
        ]
        .spacing(constant::spacing::LARGE)
        .into()
    }
}
