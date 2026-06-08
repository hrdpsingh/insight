use crate::{
    app::Message,
    components::{card, graph::Graph, title},
};
use iced::{
    Color, Element, Length,
    widget::{Space, column, responsive, row, rule, text},
};

pub fn view<'a>(
    cpu_usage_history: &[Vec<f32>],
    cpu_name: &str,
    cpu_architecture: &str,
) -> Element<'a, Message> {
    let cpu_usage_history = cpu_usage_history.to_vec();
    let cpu_name = cpu_name.to_string();
    let cpu_architecture = cpu_architecture.to_string();

    responsive(move |size| {
        let available_width = size.width - 140.0;
        let column_count = ((available_width / 200.0) as usize).max(1);

        let mut cards: Vec<Element<Message>> = cpu_usage_history
            .iter()
            .enumerate()
            .map(|(i, core_history)| {
                let cpu_usage = match core_history.last() {
                    Some(usage) => format!("{:.2}%", usage),
                    None => "Unavailable".to_string(),
                };

                let core = column![
                    Graph::new(
                        100.0,
                        Color::from_rgb8(215, 235, 255),
                        Color::from_rgb8(55, 155, 255),
                        Color::from_rgb8(175, 215, 255),
                        core_history.to_vec(),
                        100.0
                    ),
                    row![
                        text(format!("Core {}", i + 1)),
                        Space::new().width(Length::Fill),
                        text!("{}", cpu_usage),
                    ]
                ]
                .spacing(10);

                card::view(Length::Fill, core)
            })
            .collect();

        let mut rows = column![title::view("Usage")].spacing(20);
        while !cards.is_empty() {
            let take = column_count.min(cards.len());
            let mut row = row![].spacing(20);
            for card in cards.drain(..take) {
                row = row.push(card);
            }
            for _ in 0..(column_count - take) {
                row = row.push(Space::new().width(Length::Fill));
            }
            rows = rows.push(row);
        }

        title::view("Information");
        let cpu_info = column![
            row![
                text("Model").size(14),
                Space::new().width(Length::Fill),
                text(cpu_name.clone()).size(14),
            ],
            rule::horizontal(1).style(|theme| {
                rule::Style {
                    color: Color::from_rgb8(235, 240, 245),
                    ..Default::default(),
                }
            }),
            row![
                text("Architecture").size(14),
                Space::new().width(Length::Fill),
                text(cpu_architecture.clone()).size(14),
            ],
        ]
        .spacing(10);

        rows = rows.push(card::view(Length::Fill, cpu_info));
        rows.into()
    })
    .into()
}
