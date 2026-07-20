use crate::{
    components, layout, metrics,
    palette::Palette,
    state::{Cpu, Insight, Memory, Network, Processes, Storage},
};
use iced::{
    Background, Element, Length, Subscription, Theme, alignment, padding, time,
    widget::{column, container, responsive, row},
};
use std::time::Duration;
use sysinfo::{Disks, Networks, System};

#[derive(Clone)]
pub enum Message {
    Tick,
    Previous,
    Next,
    Refresh,
    Change(Theme),
}

impl Insight {
    pub fn default() -> Self {
        let system = System::new_with_specifics(metrics::refresh_system());
        let disks = Disks::new_with_refreshed_list_specifics(metrics::refresh_disks());
        let networks = Networks::new();

        let mut insight = Self {
            cpu: Cpu {
                name: system
                    .cpus()
                    .first()
                    .map(|cpu| cpu.brand().to_string())
                    .unwrap_or_else(|| "Unavailable".to_string()),
                architecture: System::cpu_arch().to_string(),
                core_count: system.cpus().len(),
                history: vec![0.0; 60],
            },
            memory: Memory { used: 0, total: 0 },
            processes: Processes {
                list: Vec::new(),
                page: 1,
            },
            storage: Storage {
                total: 0,
                used: 0,
                time: "Unavailable".to_string(),
            },
            network: Network {
                incoming: 0,
                outgoing: 0,
                receiving: false,
                sending: false,
                received: 0,
                sent: 0,
            },
            system,
            disks,
            networks,
            theme: Theme::Light,
        };

        metrics::update_memory(&mut insight);
        metrics::update_processes(&mut insight);
        metrics::update_storage(&mut insight);

        insight
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.system.refresh_specifics(metrics::refresh_system());

                metrics::update_cpu(self);
                metrics::update_memory(self);
                metrics::update_processes(self);
                metrics::update_network(self);
            }
            Message::Previous => {
                if self.processes.page > 1 {
                    self.processes.page -= 1;
                }
            }
            Message::Next => {
                if self.processes.page < self.processes.list.len().div_ceil(11) {
                    self.processes.page += 1;
                }
            }
            Message::Refresh => {
                metrics::update_storage(self);
            }
            Message::Change(theme) => {
                self.theme = theme;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            row![
                container(components::card::view(
                    column![
                        components::button::view(
                            components::svg::view(
                                include_bytes!("../icons/light_mode.svg").as_ref()
                            ),
                            match self.theme {
                                Theme::Light => None,
                                _ => Some(Message::Change(Theme::Light)),
                            },
                            true
                        ),
                        components::button::view(
                            components::svg::view(
                                include_bytes!("../icons/dark_mode.svg").as_ref()
                            ),
                            match self.theme {
                                Theme::Dark => None,
                                _ => Some(Message::Change(Theme::Dark)),
                            },
                            true
                        ),
                    ]
                    .spacing(4),
                    padding::all(4.0),
                    Length::Shrink,
                    Length::Shrink,
                    false,
                ))
                .align_x(alignment::Horizontal::Center)
                .padding(padding::top(16.0))
                .height(Length::Fill)
                .width(Length::Fixed(60.0))
                .style(move |theme| container::Style::default()
                    .background(Palette::from(theme).surface)),
                components::scroll::view(responsive(|size| {
                    container(layout::view(self, size))
                        .align_x(iced::Alignment::Center)
                        .width(Length::Fill)
                        .padding(24)
                        .into()
                }))
            ]
            .spacing(0),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |theme| container::Style {
            background: Some(Background::Color(Palette::from(theme).background)),
            ..container::Style::default()
        })
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
