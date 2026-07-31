use crate::{
    components::{self, separator::Orientation},
    constant, layout, metric,
    palette::Palette,
    state::{Cpu, Insight, Memory, Mode, Network, Processes, Storage},
    utility::{load_configuration, save_configuration},
};
use iced::{
    Element, Length, Subscription, alignment, padding, time,
    widget::{column, container, responsive, row},
};
use std::{collections::VecDeque, time::Duration};
use sysinfo::{Disks, Networks, System};

#[derive(Clone)]
pub enum Message {
    Tick,
    Previous,
    Next,
    Refresh,
    Change(Mode),
    Input(String),
}

impl Insight {
    pub fn default() -> Self {
        let system = System::new_with_specifics(metric::refresh_system());
        let disks = Disks::new_with_refreshed_list_specifics(metric::refresh_disks());
        let networks = Networks::new();

        let configuration = load_configuration().unwrap_or_default();

        let mut insight = Self {
            cpu: Cpu {
                name: system
                    .cpus()
                    .first()
                    .map_or_else(|| "Unavailable".to_string(), |cpu| cpu.brand().to_string()),
                architecture: System::cpu_arch().clone(),
                core_count: system.cpus().len(),
                history: VecDeque::from(vec![0.0; constant::graph::HISTORY_LENGTH]),
            },
            memory: Memory { used: 0, total: 0 },
            processes: Processes {
                list: Vec::new(),
                page: 1,
                search_term: String::new(),
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
            mode: configuration.mode,
        };

        metric::update_memory(&mut insight);
        metric::update_processes(&mut insight);
        metric::update_storage(&mut insight);

        insight
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.system.refresh_specifics(metric::refresh_system());

                metric::update_cpu(self);
                metric::update_memory(self);
                metric::update_processes(self);
                metric::update_network(self);
            }
            Message::Previous => {
                if self.processes.page > 1 {
                    self.processes.page -= 1;
                }
            }
            Message::Next => {
                let query = self.processes.search_term.trim().to_lowercase();
                let process_count = self
                    .processes
                    .list
                    .iter()
                    .filter(|process| {
                        query.is_empty()
                            || process.name.to_lowercase().contains(&query)
                            || process.pid.to_string().contains(&query)
                    })
                    .count();

                let maximum_pages = process_count
                    .div_ceil(crate::constant::process::COUNT)
                    .max(1);
                if self.processes.page < maximum_pages {
                    self.processes.page += 1;
                }
            }
            Message::Refresh => {
                metric::update_storage(self);
            }
            Message::Change(mode) => {
                self.mode = mode;
                match save_configuration(&self.mode) {
                    Ok(()) => {}
                    Err(error) => eprintln!("Failed to save configuration: {error}"),
                }
            }
            Message::Input(search_term) => {
                self.processes.search_term = search_term;
                self.processes.page = 1;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(row![
            container(
                column![
                    components::button::view(
                        components::svg::view(
                            include_bytes!("../assets/icons/system.svg").as_ref()
                        ),
                        match self.mode {
                            Mode::System => None,
                            _ => Some(Message::Change(Mode::System)),
                        },
                        true,
                    ),
                    components::button::view(
                        components::svg::view(
                            include_bytes!("../assets/icons/light_mode.svg").as_ref()
                        ),
                        match self.mode {
                            Mode::Light => None,
                            _ => Some(Message::Change(Mode::Light)),
                        },
                        true,
                    ),
                    components::button::view(
                        components::svg::view(
                            include_bytes!("../assets/icons/dark_mode.svg").as_ref()
                        ),
                        match self.mode {
                            Mode::Dark => None,
                            _ => Some(Message::Change(Mode::Dark)),
                        },
                        true,
                    ),
                ]
                .spacing(constant::spacing::MEDIUM)
            )
            .align_x(alignment::Horizontal::Center)
            .padding(padding::top(constant::padding::MEDIUM))
            .height(Length::Fill)
            .width(Length::Fixed(60.0))
            .style(
                move |theme| container::Style::default().background(Palette::from(theme).surface)
            ),
            components::separator::view(&Orientation::Vertical),
            components::scroll::view(responsive(|size| {
                container(layout::view(self, size))
                    .align_x(iced::Alignment::Center)
                    .width(Length::Fill)
                    .padding(constant::padding::LARGE)
                    .into()
            }))
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |theme| container::Style::default().background(Palette::from(theme).background))
        .into()
    }

    pub fn subscription(_: &Self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
