use crate::{
    app::Message,
    components::{card, scroll, title},
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
        table::column(header("PID"), |process: &Process| {
            cell(process.pid.to_string())
        })
        .width(50.0),
        table::column(header("Name"), |process: &Process| {
            cell(process.name.clone())
        })
        .width(150.0),
        table::column(header("Memory"), |process: &Process| {
            cell(format!("{} MB", process.memory))
        })
        .width(100.0),
    ];

    let table = scroll::view(
        Table::new(columns, &probe.processes)
            .width(350.0)
            .separator(0),
    );

    card::view(
        column![
            title::view("Processes"),
            container(table),
        ]
        .spacing(12)
        .height(Length::Fill),
        Length::Shrink
    )
}

fn header(label: &str) -> Element<'_, Message> {
    text(label).color(Color::from_rgb8(50, 150, 250)).into()
}

fn cell<'a>(content: String) -> Element<'a, Message> {
    tooltip(
        container(
            text(content.clone())
                .size(13)
                .font(Font::MONOSPACE)
                .color(Color::from_rgb8(0, 0, 0)),
        )
        .clip(true),
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
