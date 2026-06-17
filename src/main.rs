mod app;
mod components;
mod dashboard;
mod metrics;
mod state;
use iced::{Color, Size, window};
use state::Probe;

fn main() -> iced::Result {
    iced::application(Probe::default, Probe::update, Probe::view)
        .subscription(Probe::subscription)
        .title("Probe")
        .style(|_, _| iced::theme::Style {
            background_color: Color::from_rgba8(240, 245, 250, 0.7),
            text_color: Color::from_rgb8(0, 0, 0),
        })
        .window(window::Settings {
            size: iced::Size::new(960.0, 540.0),
            min_size: Some(Size::new(960.0, 540.0)),
            transparent: true,
            blur: true,
            ..Default::default()
        })
        .run()
}
