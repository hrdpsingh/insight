mod app;
mod components;
mod metrics;
mod models;
mod pages;
mod sidebar;
use app::Probe;
use iced::{Size, window};

fn main() -> iced::Result {
    iced::application(Probe::default, Probe::update, Probe::view)
        .subscription(Probe::subscription)
        .title("Probe")
        .window(window::Settings {
            min_size: Some(Size::new(600.0, 400.0)),
            ..Default::default()
        })
        .run()
}
