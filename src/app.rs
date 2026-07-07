use crate::{
    components::{self, scroll},
    metrics,
    palette::Palette,
    sections,
    state::{ExtendTheme, Insight, Mode},
};
use iced::{
    Alignment, Background, Color, Element, Length, Shadow, Subscription, Theme, Vector, border,
    padding, time,
    widget::{column, container, row, rule, svg},
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
                container(
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
                    .spacing(16)
                )
                .padding(padding::vertical(20))
                .align_x(Alignment::Center)
                .height(Length::Fill)
                .width(Length::Fixed(52.0))
                .style(|theme: &Theme| container::Style {
                    background: Some(Background::Color(theme.custom().surface)),
                    shadow: Shadow {
                        color: theme.custom().shadow,
                        offset: Vector::new(1.0, 1.0),
                        blur_radius: 4.0,
                    },
                    ..container::Style::default()
                }),
                rule::vertical(2).style(|theme: &Theme| rule::Style {
                    color: Color {
                        r: theme.custom().surface.r * 1.15,
                        g: theme.custom().surface.g * 1.15,
                        b: theme.custom().surface.b * 1.15,
                        a: theme.custom().surface.a,
                    },
                    fill_mode: rule::FillMode::Full,
                    radius: border::radius(0.0),
                    snap: true,
                }),
                scroll::view(
                    container(
                        column![
                            row![
                                sections::cpu::view(self),
                                sections::memory::view(self),
                                sections::processes::view(self)
                            ]
                            .spacing(24),
                            row![sections::storage::view(self), sections::network::view(self)]
                                .spacing(24),
                        ]
                        .spacing(24),
                    )
                    .width(Length::Fill)
                    .padding(24),
                )
            ]
            .spacing(0),
        )
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
