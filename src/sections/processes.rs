use crate::{
    app::Message,
    components::{self, button, card},
    metrics::format_bytes,
    state::ExtendTheme,
    state::Insight,
};

use iced::{
    Element, Font, Length, Theme,
    alignment::Vertical,
    font::Weight,
    padding,
    widget::{Space, column, container, row, svg, text, tooltip},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    let count = 5;
    let pages = insight.processes.list.len().div_ceil(count);

    let displayed_processes: Vec<_> = insight
        .processes
        .list
        .iter()
        .skip((insight.processes.page - 1) * count)
        .take(count)
        .collect();

    let table = row![
        build_column(
            "PID",
            displayed_processes
                .iter()
                .map(|process| process.pid.to_string())
                .collect(),
            60.0
        ),
        build_column(
            "Name",
            displayed_processes
                .iter()
                .map(|process| process.name.clone())
                .collect(),
            120.0
        ),
        build_column(
            "Memory",
            displayed_processes
                .iter()
                .map(|process| format_bytes(process.memory))
                .collect(),
            96.0
        ),
        build_column(
            "CPU",
            displayed_processes
                .iter()
                .map(|process| match process.cpu {
                    0.0 => "0%".to_string(),
                    _ => format!("{:.1}%", process.cpu),
                })
                .collect(),
            60.0
        ),
    ]
    .spacing(8);

    let navigation = row![
        Space::new().width(Length::Fill),
        card::view(
            row![
                button::view(
                    svg(svg::Handle::from_memory(
                        include_bytes!("../../icons/left_arrow.svg").as_ref()
                    )),
                    (insight.processes.page > 1).then_some(Message::Previous)
                ),
                text(format!("{} of {}", insight.processes.page, pages))
                    .wrapping(text::Wrapping::None),
                button::view(
                    svg(svg::Handle::from_memory(
                        include_bytes!("../../icons/right_arrow.svg").as_ref()
                    )),
                    (insight.processes.page < pages).then_some(Message::Next)
                ),
            ]
            .align_y(Vertical::Center)
            .spacing(12),
            Length::Shrink,
            |palette| palette.elevated,
            padding::all(8)
        ),
        Space::new().width(Length::Fill),
    ];

    card::view(
        column![
            row![
                Space::new().width(Length::Fill),
                components::title::view(format!("Processes - {}", insight.processes.list.len())),
                Space::new().width(Length::Fill),
            ],
            column![table, navigation].spacing(12).width(Length::Shrink)
        ]
        .spacing(16),
        Length::Shrink,
        |palette| palette.surface,
        padding::all(20.0),
    )
}

fn build_column<'a>(name: &'a str, items: Vec<String>, width: f32) -> Element<'a, Message> {
    let mut column = column![
        container(
            text(name)
                .wrapping(text::Wrapping::None)
                .font(Font {
                    weight: Weight::Bold,
                    ..Font::DEFAULT
                })
                .style(move |theme: &Theme| text::Style {
                    color: Some(theme.custom().accent),
                }),
        )
        .clip(true)
        .width(Length::Fixed(width))
        .padding(8)
    ];

    for item in items {
        column = column.push(components::tooltip::view(
            container(text(item.clone()).wrapping(text::Wrapping::None))
                .width(Length::Fixed(width))
                .padding(8)
                .clip(true),
            text(item),
            tooltip::Position::Bottom,
        ));
    }

    column.width(Length::Fixed(width)).into()
}
