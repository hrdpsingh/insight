use crate::app::Message;
use crate::components::{category, graph::Graph};
use iced::widget::{canvas, column};
use iced::{Color, Element, Length};

pub fn view<'a>() -> Element<'a, Message> {
    let mock_data = vec![10.0, 40.0, 20.0, 10.0];

    column![category::view(
        "CPU",
        column![
            canvas(Graph::new(
                mock_data,
                100.0,
                Color::from_rgb8(50, 150, 250),
                Color::from_rgb8(210, 230, 250),
                Color::from_rgb8(170, 210, 250),
            ))
            .width(Length::Fill)
            .height(Length::Fixed(200.0))
        ]
    )]
    .spacing(20)
    .into()
}
