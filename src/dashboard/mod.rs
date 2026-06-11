pub mod cpu;
pub mod processes;

use crate::{app::Message, state::Probe};
use iced::{
    Color, Element,
    widget::{container, row, rule},
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    row![
        processes::view(probe),
        rule::vertical(1).style(|_| rule::Style {
            color: Color::from_rgb8(205, 210, 215),
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: true,
        }),
        container(cpu::view(probe)).padding(30)
    ]
    .into()
}
