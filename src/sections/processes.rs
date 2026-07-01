use crate::{
    app::Message,
    components::{self, button, card},
    state::Insight,
    utilities,
};

use iced::{
    Color, Element, Font, Length,
    alignment::Vertical,
    font::Weight,
    padding,
    widget::{Space, column, container, row, text, tooltip},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    let count = 6;
    let pages = insight.processes.len().div_ceil(count);

    let displayed_processes: Vec<_> = insight
        .processes
        .iter()
        .skip((insight.page - 1) * count)
        .take(count)
        .collect();

    let table = row![
        build_column(
            "PID",
            displayed_processes
                .iter()
                .map(|process| process.pid.to_string())
                .collect(),
            80.0
        ),
        build_column(
            "Name",
            displayed_processes
                .iter()
                .map(|process| process.name.clone())
                .collect(),
            148.0
        ),
        build_column(
            "Memory",
            displayed_processes
                .iter()
                .map(|process| format!("{:.1} MB", utilities::to_mb(process.memory)))
                .collect(),
            108.0
        ),
    ]
    .spacing(8);

    let navigation = row![
        Space::new().width(Length::Fill),
        card::view(
            row![
                button::view("Back", (insight.page > 1).then_some(Message::Previous)),
                text(format!("{} of {}", insight.page, pages)).wrapping(text::Wrapping::None),
                button::view("Next", (insight.page < pages).then_some(Message::Next)),
            ]
            .align_y(Vertical::Center)
            .spacing(12),
            Length::Shrink,
            Color::from_rgb8(245, 245, 255),
            padding::all(12)
        ),
        Space::new().width(Length::Fill),
    ];

    card::view(
        column![
            row![
                Space::new().width(Length::Fill),
                components::title::view(format!("Processes - {}", insight.processes.len())),
                Space::new().width(Length::Fill),
            ],
            column![table, navigation].spacing(12).width(Length::Shrink)
        ]
        .spacing(16),
        Length::Shrink,
        Color::from_rgb8(240, 240, 250),
        padding::all(20.0),
    )
}

fn build_column<'a>(name: &'a str, items: Vec<String>, width: f32) -> Element<'a, Message> {
    let mut column = column![
        container(
            text(name)
                .color(Color::from_rgb8(150, 150, 255))
                .wrapping(text::Wrapping::None)
                .font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                }),
        )
        .clip(true)
        .width(Length::Fixed(width))
        .padding(8)
    ];

    for item in items {
        column = column.push(tooltip(
            container(text(item.clone()).wrapping(text::Wrapping::None))
                .width(Length::Fixed(width))
                .padding(8)
                .clip(true),
            card::view(
                text(item),
                Length::Shrink,
                Color::from_rgb8(245, 245, 255),
                padding::all(8),
            ),
            tooltip::Position::Bottom,
        ));
    }

    column.width(Length::Fixed(width)).into()
}
