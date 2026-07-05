use crate::{
    app::Message,
    components::{self, card},
    palette,
    state::Insight,
};
use iced::{
    Element, Length, alignment, padding,
    widget::{Space, column, row, svg, text, tooltip},
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
                components::title::view("Storage".to_string()),
                Space::new().width(Length::Fill),
                components::tooltip::view(
                    components::button::view(
                        svg(svg::Handle::from_memory(
                            include_bytes!("../../icons/refresh.svg").as_ref()
                        )),
                        Option::Some(Message::Refresh)
                    ),
                    components::constant::view("Last Refreshed", insight.storage.time.clone()),
                    tooltip::Position::Top
                )
            ]
            .align_y(alignment::Vertical::Center),
            column![
                row![
                    text("Usage"),
                    Space::new().width(Length::Fill),
                    text(format!("{:.1}%", (used_space * 100.0) / total_space))
                ],
                components::bar::view(
                    used_space,
                    total_space,
                    palette::ACCENT,
                    palette::ACCENT_LIGHT,
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
        palette::CARD,
        padding::all(20.0),
    )
}
