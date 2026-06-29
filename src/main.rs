mod app;
mod cards;
mod components;
mod metrics;
mod state;

use iced::{Size, window};
use state::Insight;

fn main() -> iced::Result {
    iced::application(Insight::default, Insight::update, Insight::view)
        .subscription(Insight::subscription)
        .title("Insight")
        .window(window::Settings {
            size: iced::Size::new(800.0, 450.0),
            min_size: Some(Size::new(800.0, 450.0)),
            ..Default::default()
        })
        .run()
}
