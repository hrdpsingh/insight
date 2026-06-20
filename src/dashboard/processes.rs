use crate::{
    app::Message,
    components::{card, scroll},
    state::Probe,
};

use iced::{
    Background, Border, Color, Element, Font, Length,
    font::Weight,
    widget::{column, container, row, text, tooltip},
};

pub fn view<'a>(probe: &'a Probe) -> Element<'a, Message> {
    let header_row = row![
        header("PID", 80.0),
        header("Name", 148.0),
        header("Memory", 108.0),
        header("CPU", 100.0)
    ]
    .spacing(8);

    let rows: Vec<Element<'a, Message>> = probe
        .processes
        .iter()
        .map(|process| {
            row![
                cell(process.pid.to_string(), 80.0),
                cell(process.name.clone(), 148.0),
                cell(format!("{} MB", process.memory), 108.0),
                cell(format!("{:.1}%", process.cpu), 100.0)
            ]
            .spacing(8)
            .into()
        })
        .collect();

    let content = column![header_row, scroll::view(column(rows))];

    card::view(content, Length::Shrink)
}

fn header<'a>(column_name: &'a str, width: f32) -> Element<'a, Message> {
    container(
        text(column_name)
            .color(Color::from_rgb8(55, 155, 255))
            .size(16)
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            }),
    )
    .width(Length::Fixed(width))
    .padding(8)
    .into()
}

fn cell<'a>(content: String, width: f32) -> Element<'a, Message> {
    tooltip(
        container(text(content.clone()).size(16))
            .width(Length::Fixed(width))
            .padding(8)
            .clip(true),
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
