use iced::{Element, Length, alignment, widget::column};

use crate::{
    app::Message,
    components::{self, card, donut, separator::Orientation},
    constant,
    metric::format_bytes,
    state::Insight,
};

pub fn view(insight: &Insight) -> Element<'_, Message> {
    card::view(
        column![
            components::title::view("Memory".to_string()),
            column![
                column![donut::view(insight.memory.used, insight.memory.total, 12.0)]
                    .width(Length::Fill)
                    .align_x(alignment::Horizontal::Center),
                column![
                    components::inline::view(
                        "Free",
                        format_bytes(insight.memory.total - insight.memory.used)
                    ),
                    components::separator::view(&Orientation::Horizontal),
                    components::inline::view("Used", format_bytes(insight.memory.used)),
                    components::separator::view(&Orientation::Horizontal),
                    components::inline::view("Total", format_bytes(insight.memory.total)),
                ]
                .spacing(constant::spacing::SMALL)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
            ]
            .spacing(constant::spacing::MEDIUM),
        ]
        .spacing(constant::spacing::LARGE),
        Length::Fixed(constant::card::HEIGHT_MEDIUM),
    )
}
