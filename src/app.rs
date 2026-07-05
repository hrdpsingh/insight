use crate::{components::scroll, metrics, palette, sections, state::Insight};
use iced::{
    Background, Element, Length, Subscription, time,
    widget::{column, container, row},
};
use std::time::Duration;

#[derive(Clone)]
pub enum Message {
    Tick,
    Previous,
    Next,
    Refresh,
}

impl Insight {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.system.refresh_specifics(metrics::refresh_system());
                self.disks
                    .refresh_specifics(false, metrics::refresh_disks());

                metrics::update_cpu(self);
                metrics::update_memory(self);
                metrics::update_processes(self);
            }
            Message::Previous => {
                if self.processes.page > 1 {
                    self.processes.page -= 1;
                }
            }
            Message::Next => {
                let count = self.processes.list.len().div_ceil(6);

                if self.processes.page < count {
                    self.processes.page += 1;
                }
            }
            Message::Refresh => {
                metrics::update_storage(self);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(scroll::view(
            container(
                column![
                    row![
                        sections::cpu::view(self),
                        sections::memory::view(self),
                        sections::processes::view(self)
                    ]
                    .spacing(24),
                    sections::storage::view(self)
                ]
                .spacing(24),
            )
            .width(Length::Fill)
            .padding(24),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(palette::BACKGROUND)),
            ..container::Style::default()
        })
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
