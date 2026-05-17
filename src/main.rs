mod app;
mod components;
mod pages;
mod state;
use state::Probe;

fn main() -> iced::Result {
    iced::application(Probe::default, Probe::update, Probe::view)
        .subscription(Probe::subscription)
        .run()
}
