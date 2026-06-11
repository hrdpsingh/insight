use crate::{
    app::Message,
    components::{scroll, title},
    state::{Probe, Process},
};

use iced::{
    Background, Border, Color, Element, Font, Length,
    widget::{
        column, container,
        table::{self, Table},
        text, tooltip,
    },
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    let columns = vec![
        table::column(header("PID"), |entry: &Process| cell(entry.pid.to_string())).width(50.0),
        table::column(header("Name"), |entry: &Process| cell(entry.name.clone())).width(150.0),
        table::column(header("Memory"), |entry: &Process| {
            cell(format!("{} MB", entry.memory_kb / 1024))
        })
        .width(100.0),
    ];

    let table: Element<'a, Message> = Table::new(columns, &probe.processes.entries)
        .width(350.0)
        .separator(0)
        .into();

    let scrollable_table = scroll::view(table);

    container(
        column![
            title::view("Processes"),
            container(scrollable_table)
                .padding(15)
                .style(|_| container::Style {
                    background: Some(Background::Color(Color::from_rgb8(240, 245, 250))),
                    border: Border {
                        color: Color::from_rgb8(205, 210, 215),
                        width: 1.0,
                        radius: 10.0.into(),
                    },
                    ..container::Style::default()
                }),
        ]
        .spacing(12)
        .height(Length::Fill),
    )
    .padding(20)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
        ..container::Style::default()
    })
    .into()
}

fn header(label: &str) -> Element<'_, Message> {
    text(label).color(Color::from_rgb8(100, 100, 100)).into()
}

fn cell<'a>(content: String) -> Element<'a, Message> {
    let inner = container(
        text(content.clone())
            .size(13)
            .font(Font::MONOSPACE)
            .color(Color::from_rgb8(0, 0, 0)),
    )
    .clip(true);

    tooltip(
        inner,
        container(text(content).size(13))
            .padding(8)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
                border: Border {
                    color: Color::from_rgb8(205, 210, 215),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                ..container::Style::default()
            }),
        tooltip::Position::Bottom,
    )
    .into()
}
