use crate::{
    Probe,
    components::sidebar,
    pages::{firmware, hardware, software},
    state::Page,
};

use iced::{
    Color, Element, Length, Subscription,
    widget::{container, row},
};
use std::time::Duration;
use sysinfo::System;

#[derive(Clone)]
pub enum Message {
    Navigate(Page),
    Tick(f32),
}

impl Probe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(page) => {
                self.page = page;
            }
            Message::Tick(cpu_usage) => {
                self.cpu_usage_history.push(cpu_usage);
                self.cpu_usage_history.remove(0);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match self.page {
            Page::Software => software::view(),
            Page::Hardware => hardware::view(&self.cpu_usage_history),
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
        iced::time::every(Duration::from_secs(1)).map(|_| {
            let mut sys = System::new_all();
            sys.refresh_cpu_all();

            std::thread::sleep(std::time::Duration::from_millis(100));
            sys.refresh_cpu_all();

            let global_cpu = sys.global_cpu_usage();
            Message::Tick(global_cpu)
        })
    }
}
