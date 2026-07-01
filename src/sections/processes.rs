use crate::{
    app::Message,
    components::{button, card},
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
                cell(format!("{:.1} MB", utilities::to_mb(process.memory)), 108.0,),
            ]
            .spacing(8)
            .into()
        })
        .collect();

    let pages = insight.processes.len().div_ceil(count);
    let navigation = row![
        Space::new().width(Length::Fill),
        card::view(
            row![
                button::view("Back", (insight.page > 1).then_some(Message::Previous)),
                text(format!("{} of {}", insight.page, pages)).wrapping(text::Wrapping::None),
                button::view("Next", (insight.page < pages).then_some(Message::Next)),
            ]
            .align_y(Vertical::Center)
            .spacing(8),
            Length::Shrink,
            Color::from_rgb8(245, 245, 255),
            padding::all(4)
        ),
        Space::new().width(Length::Fill),
    ];

    let content = column![
        header_row,
        column(rows),
        Space::new().height(Length::Fixed(12.0)),
        navigation
    ];

    card::view(
        content,
        Length::Shrink,
        Color::from_rgb8(240, 240, 250),
        padding::all(20.0),
    )
}

fn header<'a>(column_name: &'a str, width: f32) -> Element<'a, Message> {
    container(
        text(column_name)
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
    .into()
}

fn cell<'a>(content: String, width: f32) -> Element<'a, Message> {
    tooltip(
        container(text(content.clone()).wrapping(text::Wrapping::None))
            .width(Length::Fixed(width))
            .padding(8)
            .clip(true),
        card::view(
            text(content),
            Length::Shrink,
            Color::from_rgb8(245, 245, 255),
            padding::all(8),
        ),
        tooltip::Position::Bottom,
    )
    .into()
}
