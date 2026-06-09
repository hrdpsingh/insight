mod app;
mod components;
mod dashboard;
mod metrics;
mod models;
use app::Probe;
use iced::{Size, window};

fn main() -> iced::Result {
    iced::application(Probe::default, Probe::update, Probe::view)
        .subscription(Probe::subscription)
        .title("Probe")
        .window(window::Settings {
            size: iced::Size::new(1000.0, 400.0),
            min_size: Some(Size::new(1000.0, 400.0)),
            ..Default::default()
        })
        .run()
}
