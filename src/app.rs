use crate::{
    dashboard,
    metrics::{get_core_count, update_cpu_usage},
    models::Page,
};
use iced::{Color, Element, Length, Subscription, time, widget::container};
use std::time::Duration;
use sysinfo::System;

pub struct Probe {
    pub page: Page,
    pub system: System,
    pub cpu_usage_history: Vec<f32>,
    pub cpu_name: String,
    pub cpu_architecture: String,
}

impl Default for Probe {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            page: Page::default(),
            cpu_name: system
                .cpus()
                .first()
                .map(|c| c.brand().to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
            cpu_architecture: System::cpu_arch().to_string(),
            system,
            cpu_usage_history: vec![0.0; 60],
        }
    }
}

#[derive(Clone)]
pub enum Message {
    Tick,
}

impl Probe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                update_cpu_usage(self);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match self.page {
            Page::Processor => dashboard::view(
                &self.cpu_usage_history,
                &self.cpu_name,
                &self.cpu_architecture,
                get_core_count(self),
            ),
        };

        container(content)
            .style(|_theme| container::Style {
                background: Some(Color::from_rgb8(240, 245, 250).into()),
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .padding(40)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
