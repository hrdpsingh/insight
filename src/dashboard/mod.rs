pub mod processor;

use crate::app::Message;
use iced::{Element, widget::row};

pub fn view<'a>(
    cpu_usage_history: &[f32],
    cpu_name: &'a str,
    cpu_architecture: &'a str,
    core_count: usize,
) -> Element<'a, Message> {
    row![
        processor::view(cpu_usage_history.to_vec(), cpu_name, cpu_architecture, core_count),
    ]
    .spacing(20)
    .into()
}
