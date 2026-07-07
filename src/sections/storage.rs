use crate::{
    app::Message,
    components::{self, card},
    metrics::format_bytes,
    state::Insight,
};
use iced::{
    Element, Length, alignment, padding,
    widget::{Space, column, row, svg, text, tooltip},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    let free: u64 = insight.storage.disks.iter().map(|disk| disk.free).sum();
    let total: u64 = insight.storage.disks.iter().map(|disk| disk.total).sum();
    let used = total - free;

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
                    components::stacked::view("Last Refreshed", insight.storage.time.clone()),
                    tooltip::Position::Top
                )
            ]
            .align_y(alignment::Vertical::Center),
            column![
                row![
                    text("Usage"),
                    Space::new().width(Length::Fill),
                    text(format!("{:.1}%", (used * 100) as f32 / total as f32))
                ],
                components::bar::view(
                    used,
                    total,
                    insight.palette().accent,
                    insight.palette().accent_light,
                    12.0,
                ),
            ]
            .spacing(8)
            .width(Length::Fixed(240.0)),
            column![
                components::stacked::view("Used", format_bytes(used)),
                components::stacked::view("Total", format_bytes(total)),
            ]
            .spacing(4),
        ]
        .spacing(24),
        Length::Shrink,
        |palette| palette.surface,
        padding::all(20.0),
    )
}
