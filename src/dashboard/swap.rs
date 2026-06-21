use iced::{
    Color, Element, Length,
    widget::{Space, column, row, text},
};

use crate::{
    app::Message,
    components::{card, donut, title},
    state::Probe,
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    card::view(
        column![
            title::view("Swap".to_string()),
            donut::view(
                probe.swap.used,
                probe.swap.total,
                Color::from_rgb8(55, 155, 255),
                Color::from_rgb8(175, 215, 255),
                16.0,
            ),
            row![
                Space::new().width(Length::Fill),
                text(format!(
                    "{:.1} GB / {:.1} GB",
                    probe.swap.used as f32 / (1024 * 1024 * 1024) as f32,
                    probe.swap.total as f32 / (1024 * 1024 * 1024) as f32
                ))
                .size(16),
                Space::new().width(Length::Fill),
            ],
        ]
        .spacing(20),
        Length::Fill,
    )
}
