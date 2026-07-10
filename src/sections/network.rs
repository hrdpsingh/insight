use crate::{
    app::Message,
    components::{card, inline, stacked, title},
    metrics::format_bytes,
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
                    svg(svg::Handle::from_memory(
                        include_bytes!("../../icons/up_arrow.svg").as_ref()
                    ))
                    .height(20)
                    .width(20)
                    .style(move |_, _| svg::Style {
                        color: Some(if insight.network.sending {
                            insight.palette().accent
                        } else {
                            insight.palette().faded
                        }),
                    }),
                    inline::view("Outgoing", format_bytes(insight.network.outgoing)),
                ]
                .spacing(8)
                .align_y(alignment::Vertical::Center),
                row![
                    svg(svg::Handle::from_memory(
                        include_bytes!("../../icons/down_arrow.svg").as_ref()
                    ))
                    .height(20)
                    .width(20)
                    .style(move |_, _| svg::Style {
                        color: Some(if insight.network.receiving {
                            insight.palette().accent
                        } else {
                            insight.palette().faded
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
