use crate::{dashboard, metrics, state::Probe};
use iced::{Color, Element, Length, Subscription, time, widget::container};
use std::time::Duration;

#[derive(Clone)]
pub enum Message {
    Tick,
}

impl Probe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                metrics::update_cpu_usage(self);
                metrics::update_processes(self);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(dashboard::view(self))
            .style(|_theme| container::Style {
                background: Some(Color::from_rgb8(240, 245, 250).into()),
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
