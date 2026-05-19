use crate::app::Message;
use crate::components::{category, graph::Graph};
use iced::widget::{Space, column, row, text};
use iced::{Color, Element, Length};

pub fn view<'a>(cpu_usage_history: &[f32]) -> Element<'a, Message> {
    let cpu_usage = match cpu_usage_history.last() {
        Some(usage) => format!("{:.2}%", usage),
        None => "Unavailable".to_string(),
    };

    column![category::view(
        "CPU",
        column![
            Graph::new(
                200.0,
                Color::from_rgb8(215, 235, 255),
                Color::from_rgb8(55, 155, 255),
                Color::from_rgb8(175, 215, 255),
                cpu_usage_history.to_vec(),
                100.0
            ),
            row![
                text("Core 1"),
                Space::new().width(Length::Fill),
                text!("Usage: {}", cpu_usage),
            ]
        ]
        .spacing(20)
    )]
    .spacing(20)
    .into()
}
