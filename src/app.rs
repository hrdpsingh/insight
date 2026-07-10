use crate::{
    components::{self, scroll},
    helper, metrics,
    palette::Palette,
    sections,
    state::{ExtendTheme, Insight, Mode},
};
use iced::{
    Background, Element, Length, Size, Subscription, Theme, padding, time, gradient, Color,
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
                let count = self.processes.list.len().div_ceil(5);

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
                scroll::view(
                    container(responsive(move |size: Size| {
                        let spacing = 24.0;
                        let item_min_width = 340.0;

                        let available_width = size.width;
                        let mut column_count = ((available_width + spacing)
                            / (item_min_width + spacing))
                            .floor() as usize;

                        let items: Vec<Element<'_, Message>> = vec![
                            sections::memory::view(self),
                            sections::cpu::view(self),
                            sections::storage::view(self),
                            sections::processes::view(self),
                            sections::network::view(self),
                        ];

                        column_count = column_count.max(1).min(items.len());
                        helper::masonry_layout(items, column_count, spacing)
                    }))
                    .width(Length::Fill)
                    .padding(24)
                )
            ]
            .spacing(0),
        )
        .padding(4)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Background::Gradient(
                            gradient::Linear::new(180_f32.to_radians())
                                .add_stop(0.0, theme.custom().background)
                                .add_stop(
                                    1.0,
                                    Color {
                                        r: theme.custom().background.r * 0.95,
                                        g: theme.custom().background.g * 0.95,
                                        b: theme.custom().background.b * 0.95,
                                        a: theme.custom().background.a,
                                    },
                                )
                                .into(),
                        )),
            ..container::Style::default()
        })
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
