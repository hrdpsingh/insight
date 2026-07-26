use crate::{
    app::Message,
    components::{self, button, card},
    metrics::format_bytes,
    palette::Palette,
    state::Insight,
};

use iced::{
    Background, Border, Element, Font, Length, Theme,
    alignment::{self, Vertical},
    font::Weight,
    padding,
    widget::{Space, column, container, row, text, text_input, tooltip},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    let count = 10;
    let query = insight.processes.search_term.trim().to_lowercase();

    let processes: Vec<_> = insight
        .processes
        .list
        .iter()
        .filter(|process| {
            if query.is_empty() {
                true
            } else {
                process.name.to_lowercase().contains(&query)
                    || process.pid.to_string().contains(&query)
            }
        })
        .collect();

    let process_count = processes.len();
    let pages = processes.len().div_ceil(count).max(1);

    let displayed_processes: Vec<_> = processes
        .into_iter()
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
    ]
    .spacing(8);

    let navigation = row![
        button::view(
            components::svg::view(include_bytes!("../../assets/icons/left_arrow.svg").as_ref()),
            (insight.processes.page > 1).then_some(Message::Previous),
            false
        ),
        text(format!("{} of {}", insight.processes.page, pages)).wrapping(text::Wrapping::None),
        button::view(
            components::svg::view(include_bytes!("../../assets/icons/right_arrow.svg").as_ref()),
            (insight.processes.page < pages).then_some(Message::Next),
            false
        ),
    ]
    .align_y(Vertical::Center)
    .spacing(12);

    card::view(
        column![
            row![
                Space::new().width(Length::Fill),
                components::title::view(format!("Processes - {}", process_count)),
                Space::new().width(Length::Fill),
            ],
            column![
                text_input("Search...", &insight.processes.search_term)
                    .on_input(Message::Input)
                    .padding(8.0)
                    .width(Length::Fill)
                    .style(|theme, status| {
                        match status {
                            text_input::Status::Focused { .. } => text_input::Style {
                                background: Background::Color(Palette::from(theme).background),
                                border: Border::default()
                                    .width(1.0)
                                    .rounded(12.0)
                                    .color(Palette::from(theme).accent),
                                icon: Palette::from(theme).text,
                                placeholder: Palette::from(theme).muted,
                                value: Palette::from(theme).text,
                                selection: Palette::from(theme).accent,
                            },
                            _ => text_input::Style {
                                background: Background::Color(Palette::from(theme).background),
                                border: Border::default()
                                    .width(1.0)
                                    .rounded(12.0)
                                    .color(Palette::from(theme).border),
                                icon: Palette::from(theme).text,
                                placeholder: Palette::from(theme).muted,
                                value: Palette::from(theme).text,
                                selection: Palette::from(theme).accent,
                            },
                        }
                    }),
                table,
                navigation,
            ]
            .spacing(12)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
        ]
        .spacing(16),
        padding::all(20.0),
        Length::Fixed(340.0),
        Length::Fixed(624.0),
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
                    color: Some(Palette::from(theme).accent),
                }),
        )
        .clip(true)
        .width(Length::Fixed(width))
        .padding(8)
    ];

    for item in items {
        column = match name {
            "Name" => column.push(components::tooltip::view(
                container(text(item.clone()).wrapping(text::Wrapping::None))
                    .width(Length::Fixed(width))
                    .padding(8)
                    .clip(true),
                text(item),
                tooltip::Position::Bottom,
            )),
            _ => column.push(
                container(text(item.clone()).wrapping(text::Wrapping::None))
                    .width(Length::Fixed(width))
                    .padding(8)
                    .clip(true),
            ),
        }
    }

    column.width(Length::Fixed(width)).into()
}
