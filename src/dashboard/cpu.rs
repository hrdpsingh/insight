use crate::components::{card, graph::Graph, title};
use crate::{app::Message, state::Probe};
use iced::widget::{Space, row};
use iced::{
    Color, Element, Font, Length,
    font::Weight,
    widget::{column, text},
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    card::view(
        column![
            title::view(probe.cpu.name.trim()),
            Graph::new(
                200.0,
                Color::from_rgb8(215, 235, 255),
                Color::from_rgb8(55, 155, 255),
                Color::from_rgb8(175, 215, 255),
                probe.cpu.history.clone(),
                100.0,
            ),
            row![
                row![
                    text("Cores: ").size(14),
                    text(probe.cpu.core_count).size(14).font(Font {
                        weight: Weight::Bold,
                        ..Font::DEFAULT
                    }),
                ],
                Space::new().width(Length::Fill),
                row![
                    text("Architecture: ").size(14),
                    text(&probe.cpu.architecture).size(14).font(Font {
                        weight: Weight::Bold,
                        ..Font::DEFAULT
                    }),
                ],
            ]
        ]
        .spacing(10),
    )
}
