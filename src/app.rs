use crate::{
    metrics::update_cpu_usage,
    models::Page,
    pages::{cpu, firmware, software}, sidebar,
};
use iced::{
    Color, Element, Length, Subscription, time,
    widget::{container, row},
};
use std::time::Duration;
use sysinfo::System;

pub struct Probe {
    pub page: Page,
    pub system: System,
    pub cpu_usage_history: Vec<Vec<f32>>,
}

impl Default for Probe {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let core_count = system.cpus().len();

        Self {
            page: Page::default(),
            system,
            cpu_usage_history: vec![vec![0.0; 60]; core_count],
        }
    }
}

#[derive(Clone)]
pub enum Message {
    Navigate(Page),
    Tick,
}

impl Probe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(page) => {
                self.page = page;
            }
            Message::Tick => {
                update_cpu_usage(self);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match self.page {
            Page::Cpu => cpu::view(&self.cpu_usage_history),
            Page::Software => software::view(),
            Page::Firmware => firmware::view(),
        };

        container(
            row![
                sidebar::view(self.page),
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
            ]
            .spacing(40),
        )
        .padding(40)
        .style(|_| container::Style::default().background(Color::from_rgb8(235, 240, 245)))
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
