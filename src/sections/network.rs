use crate::{
    app::Message,
    components::{self, card, inline, stacked, title},
    constant,
    metric::format_bytes,
    palette::Palette,
    state::Insight,
};
use iced::{
    Element, Length, alignment,
    widget::{Space, column, row, svg},
};

pub fn view(insight: &Insight) -> Element<'_, Message> {
    card::view(
        column![
            title::view("Network".to_string()),
            column![
                row![
                    components::svg::view(
                        include_bytes!("../../assets/icons/up_arrow.svg").as_ref()
                    )
                    .style(move |theme, _| svg::Style {
                        color: Some(if insight.network.sending {
                            Palette::from(theme).accent
                        } else {
                            Palette::from(theme).muted
                        }),
                    }),
                    inline::view("Outgoing", format_bytes(insight.network.outgoing)),
                ]
                .spacing(constant::spacing::SMALL)
                .align_y(alignment::Vertical::Center),
                row![
                    components::svg::view(
                        include_bytes!("../../assets/icons/down_arrow.svg").as_ref()
                    )
                    .style(move |theme, _| svg::Style {
                        color: Some(if insight.network.receiving {
                            Palette::from(theme).accent
                        } else {
                            Palette::from(theme).muted
                        }),
                    }),
                    inline::view("Incoming", format_bytes(insight.network.incoming)),
                ]
                .spacing(constant::spacing::SMALL)
                .align_y(alignment::Vertical::Center),
            ]
            .spacing(constant::spacing::MEDIUM),
            row![
                stacked::view("Sent", format_bytes(insight.network.sent)),
                Space::new().width(Length::Fill),
                stacked::view("Received", format_bytes(insight.network.received)),
            ]
            .spacing(constant::spacing::SMALL)
        ]
        .spacing(constant::spacing::LARGE),
        Length::Fixed(constant::card::HEIGHT_SMALL),
    )
}
