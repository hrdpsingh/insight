#![windows_subsystem = "windows"]

mod app;
mod components;
mod layout;
mod metrics;
mod palette;
mod sections;
mod state;

use iced::{Pixels, Size, window};
use resvg::{tiny_skia, usvg};
use state::Insight;

fn main() -> iced::Result {
    iced::application(Insight::default, Insight::update, Insight::view)
        .subscription(Insight::subscription)
        .title("Insight")
        .theme(|insight: &Insight| insight.theme.clone())
        .settings(iced::Settings {
            default_text_size: Pixels(16.0),
            vsync: true,
            ..Default::default()
        })
        .window(window::Settings {
            icon: load_icon(include_bytes!("../icons/insight.svg"), 128, 128),
            size: iced::Size::new(960.0, 540.0),
            min_size: Some(Size::new(480.0, 270.0)),
            ..Default::default()
        })
        .run()
}

fn load_icon(bytes: &[u8], width: u32, height: u32) -> Option<window::Icon> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_data(bytes, &options).ok()?;

    let transform = tiny_skia::Transform::from_scale(
        width as f32 / tree.size().width(),
        height as f32 / tree.size().height(),
    );

    let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    window::icon::from_rgba(pixmap.data().to_vec(), width, height).ok()
}
