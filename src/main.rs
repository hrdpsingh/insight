mod app;
mod components;
mod models;
mod pages;

use app::Probe;

fn main() -> iced::Result {
    iced::run(Probe::update, Probe::view)
}
