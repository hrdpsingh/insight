use crate::{
    app::Message,
    components::{self, card},
    state::Insight,
};
use iced::{
    Color, Element, Length, padding,
    widget::{Space, column, row, text},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    let total_space: f32 = insight.storage.disks.iter().map(|disk| disk.total).sum();
    let available_space: f32 = insight
        .storage
        .disks
        .iter()
        .map(|disk| disk.available)
        .sum();
    let used_space = total_space - available_space;

    card::view(
        column![
            row![
                Space::new().width(Length::Fill),
                components::title::view("Storage".to_string()),
                Space::new().width(Length::Fill),
            ],
            column![
                row![
                    text("Usage"),
                    Space::new().width(Length::Fill),
                    text(format!("{:.1}%", (used_space * 100.0) / total_space))
                ],
                components::bar::view(
                    used_space,
                    total_space,
                    Color::from_rgb8(150, 150, 255),
                    Color::from_rgb8(200, 200, 255),
                    12.0,
                ),
            ]
            .spacing(8)
            .width(Length::Fixed(240.0)),
            column![
                components::constant::view("Total", format!("{:.1} GB", total_space)),
                components::constant::view("Used", format!("{:.1} GB", used_space)),
            ]
            .spacing(8),
        ]
        .spacing(24),
        Length::Shrink,
        Color::from_rgb8(240, 240, 250),
        padding::all(20.0),
    )
}
