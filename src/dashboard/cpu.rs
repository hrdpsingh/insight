use crate::components::title;
use crate::components::{card, graph};
use crate::{app::Message, state::Probe};
use iced::widget::{Space, text};
use iced::widget::{column, row};
use iced::{Color, Element, Length};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    column![
        title::view(format!(
            "CPU {:.1}%",
            probe.cpu.history.last().copied().unwrap_or(0.0)
        )),
        card::view(
            column![graph::view(
                probe.cpu.history.clone(),
                100.0,
                150.0,
                Color::from_rgb8(215, 235, 255),
                Color::from_rgb8(55, 155, 255),
                Color::from_rgb8(175, 215, 255),
            )]
            .spacing(8),
            Color::from_rgb8(255, 255, 255),
        ),
        column![
            card::view(row![
                text("Model").size(16),
                Space::new().width(20),
                Space::new().width(Length::Fill),
                text(probe.cpu.name.trim()).size(16)
            ],
            Color::from_rgb8(255, 255, 255),
            ),
            card::view(row![
                text("Cores").size(16),
                Space::new().width(20),
                Space::new().width(Length::Fill),
                text(probe.cpu.core_count).size(16)
            ],
            Color::from_rgb8(255, 255, 255),
            ),
            card::view(row![
                text("Architecture").size(16),
                Space::new().width(20),
                Space::new().width(Length::Fill),
                text(probe.cpu.architecture.clone()).size(16)
            ],
            Color::from_rgb8(255, 255, 255),
            ),
        ]
        .spacing(4)
    ]
    .spacing(16)
    .into()
}
