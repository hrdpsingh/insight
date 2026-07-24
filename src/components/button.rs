use crate::{app::Message, palette::Palette};
use iced::{
    Border, Element, Theme,
    widget::{Svg, button, svg},
};

pub fn view(icon: Svg<'_>, navigate: Option<Message>, sidebar: bool) -> Element<'_, Message> {
    let enabled = navigate.is_some();

    let button_widget = button(icon.height(20).width(20).style(move |theme: &Theme, _| {
        let palette = Palette::from(theme);

        svg::Style {
            color: Some(match (sidebar, enabled) {
                (_, true) => palette.muted,
                (true, false) => palette.accent,
                (false, false) => palette.muted,
            }),
        }
    }))
    .padding(4)
    .style(move |theme: &Theme, _| button::Style {
        background: match (enabled, sidebar) {
            (true, true) => Some(Palette::from(theme).background.into()),
            (false, true) => Some(Palette::from(theme).surface.into()),
            (true, false) => Some(Palette::from(theme).surface.into()),
            (false, false) => Some(Palette::from(theme).background.into()),
        },
        border: Border::default().rounded(8.0),
        ..Default::default()
    });

    match navigate {
        Some(message) => button_widget.on_press(message).into(),
        None => button_widget.into(),
    }
}
