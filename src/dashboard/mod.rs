pub mod cpu;
pub mod processes;

use crate::{app::Message, state::Probe};
use iced::{
    Element,
    widget::{container, row},
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    row![
        processes::view(probe),
        container(cpu::view(probe))
    ].padding(30).spacing(30)
    .into()
}
