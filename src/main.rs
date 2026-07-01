mod app;
mod components;
mod metrics;
mod sections;
mod state;
mod utilities;

use iced::{Pixels, Size, window};
use state::Insight;

fn main() -> iced::Result {
    iced::application(Insight::default, Insight::update, Insight::view)
        .subscription(Insight::subscription)
        .title("Insight")
        .settings(iced::Settings {
            default_text_size: Pixels(16.0),
            vsync: false,
            ..Default::default()
        })
        .window(window::Settings {
            size: iced::Size::new(800.0, 450.0),
            min_size: Some(Size::new(800.0, 450.0)),
            ..Default::default()
        })
        .run()
}
