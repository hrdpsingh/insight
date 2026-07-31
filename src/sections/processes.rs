use crate::{
    app::Message,
    components::{self, button, card},
    constant,
    utility::format_bytes,
    palette::Palette,
    state::{Insight, Process},
};

use iced::{
    Background, Border, Element, Font, Length, Theme,
    alignment::{self, Vertical},
    font::Weight,
    widget::{column, container, row, text, text_input, tooltip},
};

pub fn view(insight: &Insight) -> Element<'_, Message> {
    let (displayed, process_count, pages) = filter(insight);

    card::view(
        column![
            components::title::view(format!("Processes - {process_count}")),
            column![
                search_input(&insight.processes.search_term),
                process_table(&displayed),
                pagination_bar(insight.processes.page, pages),
            ]
            .spacing(constant::spacing::MEDIUM)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
        ]
        .spacing(constant::spacing::MEDIUM),
        Length::Fixed(constant::card::HEIGHT_LARGE),
    )
}

fn filter(insight: &Insight) -> (Vec<&Process>, usize, usize) {
    let query = insight.processes.search_term.trim().to_lowercase();

    let filtered: Vec<&Process> = insight
        .processes
        .list
        .iter()
        .filter(|process| {
            if query.is_empty() {
                true
            } else {
                process.name.to_lowercase().contains(&query)
            }
        })
        .collect();

    let process_count = filtered.len();
    let pages = process_count.div_ceil(constant::process::COUNT).max(1);

    let displayed = filtered
        .into_iter()
        .skip((insight.processes.page - 1) * constant::process::COUNT)
        .take(constant::process::COUNT)
        .collect();

    (displayed, process_count, pages)
}

fn search_input(search_term: &str) -> Element<'_, Message> {
    text_input("Search...", search_term)
        .on_input(Message::Input)
        .padding(constant::padding::SMALL)
        .width(Length::Fill)
        .style(|theme, status| {
            let text_input_style = text_input::Style {
                background: Background::Color(Palette::from(theme).background),
                border: Border::default()
                    .width(constant::border::WIDTH)
                    .rounded(constant::border::RADIUS)
                    .color(Palette::from(theme).border),
                icon: Palette::from(theme).text,
                placeholder: Palette::from(theme).muted,
                value: Palette::from(theme).text,
                selection: Palette::from(theme).accent,
            };

            match status {
                text_input::Status::Focused { .. } => text_input::Style {
                    border: text_input_style.border.color(Palette::from(theme).accent),
                    ..text_input_style
                },
                _ => text_input_style,
            }
        })
        .into()
}

fn process_table<'a>(processes: &[&'a Process]) -> Element<'a, Message> {
    row![
        build_column(
            "PID",
            processes
                .iter()
                .map(|process| process.pid.to_string())
                .collect(),
            60.0
        ),
        build_column(
            "Name",
            processes
                .iter()
                .map(|process| process.name.clone())
                .collect(),
            120.0
        ),
        build_column(
            "Memory",
            processes
                .iter()
                .map(|process| format_bytes(process.memory))
                .collect(),
            96.0
        ),
    ]
    .spacing(constant::spacing::SMALL)
    .into()
}

fn pagination_bar<'a>(current_page: usize, total_pages: usize) -> Element<'a, Message> {
    container(
        row![
            button::view(
                components::svg::view(include_bytes!("../../asset/icon/left_arrow.svg").as_ref()),
                (current_page > 1).then_some(Message::Previous),
                false
            ),
            text(format!("{current_page} of {total_pages}")).wrapping(text::Wrapping::None),
            button::view(
                components::svg::view(include_bytes!("../../asset/icon/right_arrow.svg").as_ref()),
                (current_page < total_pages).then_some(Message::Next),
                false
            ),
        ]
        .align_y(Vertical::Center)
        .spacing(constant::spacing::SMALL),
    )
    .padding(constant::padding::SMALL)
    .style(move |theme| {
        container::Style::default().border(
            Border::default()
                .rounded(constant::border::RADIUS)
                .width(constant::border::WIDTH)
                .color(Palette::from(theme).border),
        )
    })
    .into()
}

fn build_column(name: &str, items: Vec<String>, width: f32) -> Element<'_, Message> {
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
        .padding(constant::padding::SMALL)
    ];

    for item in items {
        column = match name {
            "Name" => column.push(components::tooltip::view(
                container(text(item.clone()).wrapping(text::Wrapping::None))
                    .width(Length::Fixed(width))
                    .padding(constant::padding::SMALL)
                    .clip(true),
                text(item),
                tooltip::Position::Bottom,
            )),
            _ => column.push(
                container(text(item.clone()).wrapping(text::Wrapping::None))
                    .width(Length::Fixed(width))
                    .padding(constant::padding::SMALL)
                    .clip(true),
            ),
        }
    }

    column.width(Length::Fixed(width)).into()
}
