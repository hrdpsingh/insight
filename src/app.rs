use crate::{components::scroll, dashboard, metrics, state::Probe};
use iced::{
    Background, Color, Element, Length, Subscription, time,
    widget::{column, container, row},
};
use std::time::Duration;

#[derive(Clone)]
pub enum Message {
    Tick,
    Previous,
    Next,
}

impl Probe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.system.refresh_all();

                metrics::update_cpu_usage(self);
                metrics::update_memory_usage(self);
                metrics::update_swap_usage(self);
                metrics::get_processes(self);
            }
            Message::Previous => {
                if self.page > 1 {
                    self.page -= 1;
                }
            }
            Message::Next => {
                let count = self.processes.len().div_ceil(8);

                if self.page < count {
                    self.page += 1;
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        scroll::view(
            container(
                row![
                    column![dashboard::cpu::view(self),].spacing(24),
                    column![
                        row![dashboard::memory::view(self), dashboard::swap::view(self)]
                            .spacing(24),
                        dashboard::processes::view(self)
                    ]
                    .spacing(24),
                ]
                .spacing(24),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(24)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb8(250, 250, 245))),
                ..container::Style::default()
            }),
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
