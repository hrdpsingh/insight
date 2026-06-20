use iced::{
    Color, Element,
    widget::{column, row},
};

use crate::{
    app::Message,
    components::{card, donut, title},
    state::Probe,
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    row![
        column!(
            title::view("Memory".to_string()),
            card::view(
                donut::view(
                    probe.memory.used,
                    probe.memory.total,
                    Color::from_rgb8(55, 155, 255),
                    Color::from_rgb8(175, 215, 255),
                    20.0,
                ),
                iced::Length::Fill
            ),
        ),
        column!(
            title::view("Memory".to_string()),
            card::view(
                donut::view(
                    probe.swap.used,
                    probe.swap.total,
                    Color::from_rgb8(55, 155, 255),
                    Color::from_rgb8(175, 215, 255),
                    20.0,
                ),
                iced::Length::Fill
            ),
        )
    ]
    .spacing(16)
    .into()
}
