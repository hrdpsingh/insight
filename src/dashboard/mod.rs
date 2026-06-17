pub mod cpu;
pub mod processes;

use crate::{app::Message, components::scroll, state::Probe};
use iced::{
    Color, Element,
    widget::{column, row, rule},
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    row![
        processes::view(probe),
        rule::vertical(1).style(|_| rule::Style {
            color: Color::from_rgb8(180, 185, 190),
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: true,
        }),
        scroll::view(column![cpu::view(probe)].padding(30)),
    ]
    .into()
}
