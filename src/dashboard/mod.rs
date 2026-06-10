pub mod processor;

use crate::{state::Probe, app::Message};
use iced::{Element, widget::row};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    row![
        processor::view(probe),
    ]
    .spacing(20)
    .into()
}