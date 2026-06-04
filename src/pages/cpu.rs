use crate::{app::Message, components::{card, graph::Graph}};
use iced::{Color, Element, Length, widget::{Space, column, row, text}};

pub fn view<'a>(cpu_usage_history: &[Vec<f32>]) -> Element<'a, Message> {
    let mut cores = column![].spacing(30);

    for (i, core_history) in cpu_usage_history.iter().enumerate() {
        let cpu_usage = match core_history.last() {
            Some(usage) => format!("{:.2}%", usage),
            None => "Unavailable".to_string(),
        };

        let core = column![
            Graph::new(
                200.0,
                Color::from_rgb8(215, 235, 255),
                Color::from_rgb8(55, 155, 255),
                Color::from_rgb8(175, 215, 255),
                core_history.to_vec(),
                100.0
            ),
            row![
                text(format!("Core {}", i + 1)),
                Space::new().width(Length::Fill),
                text!("Usage: {}", cpu_usage),
            ]
        ]
        .spacing(10);

        cores = cores.push(core);
    }

    column![card::view(Length::Fill, cores)].spacing(20).into()
}
