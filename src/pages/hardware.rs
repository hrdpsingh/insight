use crate::app::Message;
use crate::components::{category, graph::Graph};
use iced::widget::column;
use iced::{Color, Element};

pub fn view<'a>(cpu_usage_history: &[f32]) -> Element<'a, Message> {
    column![category::view(
        "CPU",
        column![Graph::new(
            200.0,
            Color::from_rgb8(215, 235, 255),
            Color::from_rgb8(55, 155, 255),
            Color::from_rgb8(175, 215, 255),
            cpu_usage_history.to_vec(),
            100.0
        )]
    )]
    .spacing(20)
    .into()
}
