pub mod cpu;
pub mod memory;
pub mod processes;

use crate::{app::Message, state::Probe};
use iced::{
    Element,
    widget::{column, row},
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    row![
        processes::view(probe),
        column![cpu::view(probe), memory::view(probe)].spacing(24)
    ]
    .spacing(24)
    .into()
}
