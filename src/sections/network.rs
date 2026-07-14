use crate::{
    app::Message,
    components::{self, card, inline, stacked, title},
    metrics::format_bytes,
    palette::Palette,
    state::Insight,
};
use iced::{
    Element, Length, alignment, padding,
    widget::{Space, column, row, svg},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    card::view(
        column![
            row![
                Space::new().width(Length::Fill),
                title::view("Network".to_string()),
                Space::new().width(Length::Fill),
            ],
            column![
                row![
                    components::svg::view(include_bytes!("../../icons/up_arrow.svg").as_ref())
                        .style(move |theme, _| svg::Style {
                            color: Some(if insight.network.sending {
                                Palette::from(theme).accent
                            } else {
                                Palette::from(theme).faded
                            }),
                        }),
                    inline::view("Outgoing", format_bytes(insight.network.outgoing)),
                ]
                .spacing(8)
                .align_y(alignment::Vertical::Center),
                row![
                    components::svg::view(include_bytes!("../../icons/down_arrow.svg").as_ref())
                        .style(move |theme, _| svg::Style {
                            color: Some(if insight.network.receiving {
                                Palette::from(theme).accent
                            } else {
                                Palette::from(theme).faded
                            }),
                        }),
                    inline::view("Incoming", format_bytes(insight.network.incoming)),
                ]
                .spacing(8)
                .align_y(alignment::Vertical::Center),
            ]
            .spacing(12),
            row![
                stacked::view("Sent", format_bytes(insight.network.sent)),
                Space::new().width(Length::Fill),
                stacked::view("Received", format_bytes(insight.network.received)),
            ]
            .spacing(4)
        ]
        .spacing(20),
        Length::Fixed(340.0),
        |palette| palette.surface,
        padding::all(20.0),
    )
}
