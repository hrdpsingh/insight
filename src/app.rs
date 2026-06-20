use crate::{components::scroll, dashboard, metrics, state::Probe};
use iced::{Background, Color, Element, Length, Subscription, time, widget::container};
use std::time::Duration;
use sysinfo::ProcessesToUpdate;

#[derive(Clone)]
pub enum Message {
    Tick,
}

impl Probe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                metrics::update_cpu_usage(self);
                self.system.refresh_processes(ProcessesToUpdate::All, true);
                self.processes = metrics::get_processes(&self.system);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        scroll::view(
            container(dashboard::view(self))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(28)
                .style(|_| container::Style {
                    background: Some(Background::Color(Color::from_rgb8(245, 245, 245))),
                    ..container::Style::default()
                }),
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
