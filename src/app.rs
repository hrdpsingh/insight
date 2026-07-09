use crate::{
    components::{self, scroll},
    metrics,
    palette::Palette,
    sections,
    state::{ExtendTheme, Insight, Mode},
};
use iced::{
    Alignment, Background, Element, Length, Subscription, Theme, padding, time,
    widget::{column, container, responsive, row, svg},
};
use std::time::Duration;

#[derive(Clone)]
pub enum Message {
    Tick,
    Previous,
    Next,
    Refresh,
    Change(Mode),
}

impl Insight {
    pub fn default() -> Self {
        metrics::initialize()
    }

    pub fn palette(&self) -> &'static Palette {
        self.mode.convert().custom()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.system.refresh_specifics(metrics::refresh_system());
                self.disks
                    .refresh_specifics(false, metrics::refresh_disks());

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
                let count = self.processes.list.len().div_ceil(6);

                if self.processes.page < count {
                    self.processes.page += 1;
                }
            }
            Message::Refresh => {
                metrics::update_storage(self);
            }
            Message::Change(mode) => {
                self.mode = mode;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        container(
            row![
                container(components::card::view(
                    column![
                        components::sidebar_button::view(
                            svg(svg::Handle::from_memory(
                                include_bytes!("../icons/light_mode.svg").as_ref()
                            )),
                            match self.mode {
                                Mode::Light => None,
                                Mode::Dark => Some(Message::Change(Mode::Light)),
                            }
                        ),
                        components::sidebar_button::view(
                            svg(svg::Handle::from_memory(
                                include_bytes!("../icons/dark_mode.svg").as_ref()
                            )),
                            match self.mode {
                                Mode::Dark => None,
                                Mode::Light => Some(Message::Change(Mode::Dark)),
                            }
                        ),
                    ]
                    .spacing(16),
                    Length::Shrink,
                    |palette| palette.surface,
                    padding::all(12.0),
                ))
                .padding(padding::top(24).left(24)),
                scroll::view(responsive(|size| {
                    if size.width > 800.0 {
                        container(
                            column![
                                row![
                                    sections::memory::view(self),
                                    sections::cpu::view(self),
                                    sections::storage::view(self),
                                ]
                                .align_y(Alignment::End)
                                .spacing(24),
                                row![
                                    sections::processes::view(self),
                                    sections::network::view(self),
                                ]
                                .align_y(Alignment::Start)
                                .spacing(24),
                            ]
                            .align_x(Alignment::Center)
                            .spacing(24),
                        )
                        .width(Length::Fill)
                        .center_x(Length::Fill)
                        .padding(24)
                        .into()
                    } else if size.width > 400.0 {
                        container(
                            row![
                                column![
                                    sections::memory::view(self),
                                    sections::cpu::view(self),
                                    sections::storage::view(self),
                                ]
                                .align_x(Alignment::End)
                                .spacing(24),
                                column![
                                    sections::processes::view(self),
                                    sections::network::view(self),
                                ]
                                .align_x(Alignment::Start)
                                .spacing(24),
                            ]
                            .align_y(Alignment::Center)
                            .spacing(24),
                        )
                        .width(Length::Fill)
                        .center_x(Length::Fill)
                        .padding(24)
                        .into()
                    } else {
                        container(
                                column![
                                    sections::memory::view(self),
                                    sections::cpu::view(self),
                                    sections::storage::view(self),
                                    sections::processes::view(self),
                                    sections::network::view(self),
                                ]
                                .align_x(Alignment::Start)
                                .spacing(24)
                        )
                        .width(Length::Fill)
                        .center_x(Length::Fill)
                        .padding(24)
                        .into()
                    }
                }))
            ]
            .spacing(0),
        )
        .padding(4)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Background::Color(theme.custom().background)),
            ..container::Style::default()
        })
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
