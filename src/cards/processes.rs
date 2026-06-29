use crate::{
    app::Message,
    components::{button, card},
    state::Insight,
};

use iced::{
    Background, Border, Color, Element, Font, Length, Shadow, Vector,
    alignment::Vertical,
    font::Weight,
    widget::{Space, column, container, row, text, tooltip},
};

pub fn view<'a>(insight: &'a Insight) -> Element<'a, Message> {
    let header_row = row![
        header("PID", 80.0),
        header("Name", 148.0),
        header("Memory", 108.0),
    ]
    .spacing(8);

    let count = 6;

    let rows: Vec<Element<'a, Message>> = insight
        .processes
        .iter()
        .skip((insight.page - 1) * count)
        .take(count)
        .map(|process| {
            row![
                cell(process.pid.to_string(), 80.0),
                cell(process.name.clone(), 148.0),
                cell(
                    format!("{:.1} MB", process.memory as f32 / (1024.0 * 1024.0)),
                    108.0,
                ),
            ]
            .spacing(8)
            .into()
        })
        .collect();

    let pages = insight.processes.len().div_ceil(count);
    let navigation = row![
        Space::new().width(Length::Fill),
        container(
            row![
                button::view("Back", (insight.page > 1).then_some(Message::Previous)),
                text(format!("{} of {}", insight.page, pages)).wrapping(text::Wrapping::None),
                button::view("Next", (insight.page < pages).then_some(Message::Next)),
            ]
            .align_y(Vertical::Center)
            .spacing(8)
        )
        .clip(true)
        .padding(4)
        .style(move |_| container::Style {
            background: Some(Color::from_rgb8(245, 245, 255).into()),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::from_rgb8(190, 190, 190),
                offset: Vector::new(1.0, 1.0),
                blur_radius: 4.0,
            },
            ..container::Style::default()
        }),
        Space::new().width(Length::Fill),
    ];

    let content = column![
        header_row,
        column(rows),
        Space::new().height(Length::Fixed(12.0)),
        navigation
    ];

    card::view(content, Length::Shrink)
}

fn header<'a>(column_name: &'a str, width: f32) -> Element<'a, Message> {
    container(
        text(column_name)
            .color(Color::from_rgb8(150, 150, 255))
            .size(16)
            .wrapping(text::Wrapping::None)
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            }),
    )
    .clip(true)
    .width(Length::Fixed(width))
    .padding(8)
    .into()
}

fn cell<'a>(content: String, width: f32) -> Element<'a, Message> {
    tooltip(
        container(
            text(content.clone())
                .size(16)
                .wrapping(text::Wrapping::None),
        )
        .width(Length::Fixed(width))
        .padding(8)
        .clip(true),
        container(text(content).size(16))
            .padding(8)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(245, 245, 255))),
                border: Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                shadow: Shadow {
                    color: Color::from_rgb8(190, 190, 190),
                    offset: Vector::new(1.0, 1.0),
                    blur_radius: 4.0,
                },
                ..container::Style::default()
            }),
        tooltip::Position::Bottom,
    )
    .into()
}
