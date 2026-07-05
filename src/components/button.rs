use crate::{app::Message, palette};
use iced::{
    Element,
    widget::{Svg, button, svg},
};

pub fn view(icon: Svg<'_>, navigate: Option<Message>) -> Element<'_, Message> {
    let enabled = navigate.is_some();

    let standard_color = if enabled {
        palette::MUTED
    } else {
        palette::DISABLED
    };

    let button_widget = button(
        icon.height(20)
            .width(20)
            .style(move |_, status| svg::Style {
                color: Some(match status {
                    svg::Status::Hovered if enabled => palette::ACCENT,
                    _ => standard_color,
                }),
            }),
    )
    .padding(0)
    .style(move |_, _| button::Style {
        background: Some(palette::TRANSPARENT.into()),
        ..Default::default()
    });

    match navigate {
        Some(message) => button_widget.on_press(message).into(),
        None => button_widget.into(),
    }
}
