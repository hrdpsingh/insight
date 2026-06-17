use crate::{
    app::Message,
    components::scroll,
    state::{Probe, Process},
};

use iced::{
    Background, Border, Color, Element, Font,
    font::Weight,
    widget::{
        container,
        table::{self, Table},
        text, tooltip,
    },
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    let columns = vec![
        table::column(header("PID"), |process: &Process| {
            cell(process.pid.to_string())
        })
        .width(100.0),
        table::column(header("Name"), |process: &Process| {
            cell(process.name.clone())
        })
        .width(148.0),
        table::column(header("Memory"), |process: &Process| {
            cell(format!("{} MB", process.memory))
        })
        .width(100.0),
    ];

    container(scroll::view(
        Table::new(columns, &probe.processes)
            .width(360.0)
            .separator(0)
            .padding(8),
    ))
    .padding(12)
    .style(|_| container::Style {
        background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
        ..container::Style::default()
    })
    .into()
}

fn header(label: &str) -> Element<'_, Message> {
    text(label)
        .color(Color::from_rgb8(100, 150, 255))
        .size(16)
        .font(Font {
            weight: Weight::Bold,
            ..Font::DEFAULT
        })
        .into()
}

fn cell<'a>(content: String) -> Element<'a, Message> {
    tooltip(
        container(text(content.clone()).size(16)).clip(true),
        container(text(content).size(16))
            .padding(8)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(240, 245, 250))),
                border: Border {
                    color: Color::from_rgb8(205, 210, 215),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..container::Style::default()
            }),
        tooltip::Position::Bottom,
    )
    .into()
}
