use iced::{Element, border::Radius, widget::rule};

use crate::{app::Message, constant, palette::Palette};

pub enum Orientation {
    Horizontal,
    Vertical,
}

fn rule_style(theme: &iced::Theme) -> rule::Style {
    rule::Style {
        color: Palette::from(theme).border,
        radius: Radius::new(0),
        fill_mode: rule::FillMode::Full,
        snap: false,
    }
}

pub fn view(orientation: &Orientation) -> Element<'_, Message> {
    match orientation {
        Orientation::Vertical => rule::vertical(constant::border::WIDTH)
            .style(rule_style)
            .into(),
        Orientation::Horizontal => rule::horizontal(constant::border::WIDTH)
            .style(rule_style)
            .into(),
    }
}
