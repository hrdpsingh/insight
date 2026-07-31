#![windows_subsystem = "windows"]

mod app;
mod components;
mod constant;
mod layout;
mod metric;
mod palette;
mod sections;
mod state;
mod utility;

use iced::{Pixels, Size, Theme, window};
use state::Insight;

const ICON: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/icon.rgba"));

fn main() -> iced::Result {
    iced::application(Insight::default, Insight::update, Insight::view)
        .subscription(Insight::subscription)
        .title("Insight")
        .theme(|insight: &Insight| Theme::from(insight.mode.clone()))
        .settings(iced::Settings {
            default_text_size: Pixels(16.0),
            vsync: true,
            ..Default::default()
        })
        .window(window::Settings {
            icon: window::icon::from_rgba(ICON.to_vec(), 128, 128).ok(),
            size: iced::Size::new(
                constant::window::INITIAL_WIDTH,
                constant::window::INITIAL_HEIGHT,
            ),
            min_size: Some(Size::new(
                constant::window::MINIMUM_WIDTH,
                constant::window::MINIMUM_HEIGHT,
            )),
            ..Default::default()
        })
        .run()
}
