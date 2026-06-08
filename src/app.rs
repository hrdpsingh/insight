use crate::{metrics::update_cpu_usage, models::Page, pages::processor, sidebar};
use iced::{
    Background, Color, Element, Length, Padding, Subscription, Theme, time,
    widget::{
        container, row, scrollable,
        scrollable::{Direction, Scrollbar},
    },
};
use std::time::Duration;
use sysinfo::System;

pub struct Probe {
    pub page: Page,
    pub system: System,
    pub cpu_usage_history: Vec<Vec<f32>>,
    pub cpu_name: String,
    pub cpu_architecture: String,
}

impl Default for Probe {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let core_count = system.cpus().len();

        Self {
            page: Page::default(),
            cpu_name: system
                .cpus()
                .first()
                .map(|c| c.brand().to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
            cpu_architecture: System::cpu_arch().to_string(),
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
            Page::Processor => processor::view(
                &self.cpu_usage_history,
                &self.cpu_name,
                &self.cpu_architecture,
            ),
        };

        container(
            row![
                container(sidebar::view(self.page)),
                scrollable(
                    container(content)
                        .width(Length::Fill)
                        .center_x(Length::Fill)
                        .padding(Padding {
                            top: 40.0,
                            right: 40.0,
                            bottom: 40.0,
                            left: 10.0,
                        })
                )
                .direction(Direction::Vertical(
                    Scrollbar::default().width(6).scroller_width(6),
                ))
                .style(|theme: &Theme, status: scrollable::Status| {
                    let mut base_style = scrollable::default(theme, status);

                    match status {
                        scrollable::Status::Hovered {
                            is_vertical_scrollbar_hovered,
                            ..
                        } => {
                            base_style.vertical_rail.scroller.background =
                                Background::Color(if is_vertical_scrollbar_hovered {
                                    Color::from_rgb8(160, 160, 160)
                                } else {
                                    Color::from_rgb8(180, 180, 180)
                                });
                        }
                        scrollable::Status::Dragged { .. } => {
                            base_style.vertical_rail.scroller.background =
                                Background::Color(Color::from_rgb8(160, 160, 160));
                        }
                        scrollable::Status::Active { .. } => {
                            base_style.vertical_rail.scroller.background =
                                Background::Color(Color::from_rgb8(180, 180, 180));
                        }
                    }

                    base_style
                })
            ]
            .spacing(40),
        )
        .style(|_| container::Style::default().background(Color::from_rgb8(235, 240, 245)))
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
