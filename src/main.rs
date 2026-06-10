mod app;
mod components;
mod dashboard;
mod state;
mod metrics;
use state::Probe;
use iced::{Size, window};

fn main() -> iced::Result {
    iced::application(Probe::default, Probe::update, Probe::view)
        .subscription(Probe::subscription)
        .title("Probe")
        .window(window::Settings {
            size: iced::Size::new(640.0, 360.0),
            min_size: Some(Size::new(640.0, 360.0)),
            ..Default::default()
        })
        .run()
}
