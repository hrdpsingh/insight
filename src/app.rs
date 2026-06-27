use crate::{cards, components::scroll, metrics, state::Insight};
use iced::{
    Background, Color, Element, Length, Subscription, time,
    widget::{container, row},
};
use std::time::Duration;

#[derive(Clone)]
pub enum Message {
    Tick,
    Previous,
    Next,
}

impl Insight {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.system.refresh_all();

                metrics::update_cpu_usage(self);
                metrics::update_memory_usage(self);
                metrics::get_processes(self);
            }
            Message::Previous => {
                if self.page > 1 {
                    self.page -= 1;
                }
            }
            Message::Next => {
                let count = self.processes.len().div_ceil(6);

                if self.page < count {
                    self.page += 1;
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(scroll::view(
            container(
                row![
                    cards::cpu::view(self),
                    cards::memory::view(self),
                    cards::processes::view(self)
                ]
                .spacing(24),
            )
            .padding(24),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgb8(235, 235, 245))),
            ..container::Style::default()
        })
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
