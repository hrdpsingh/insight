mod app;
mod components;
mod dashboard;
mod metrics;
mod state;
use iced::{Size, window};
use state::Probe;

fn main() -> iced::Result {
    iced::application(Probe::default, Probe::update, Probe::view)
        .subscription(Probe::subscription)
        .title("Probe")
        .window(window::Settings {
            size: iced::Size::new(960.0, 540.0),
            min_size: Some(Size::new(960.0, 540.0)),
            ..Default::default()
        })
        .run()
}
