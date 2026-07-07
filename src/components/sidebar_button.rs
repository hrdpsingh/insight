use crate::{app::Message, state::ExtendTheme};
use iced::{
    Element, Theme,
    widget::{Svg, button, svg},
};

pub fn view(icon: Svg<'_>, navigate: Option<Message>) -> Element<'_, Message> {
    let enabled = navigate.is_some();

    let button_widget = button(
        icon.height(20)
            .width(20)
            .style(move |theme: &Theme, status| {
                let palette = theme.custom();

                svg::Style {
                    color: Some(match (status, enabled) {
                        (_, true) => palette.faded,
                        (_, false) => palette.accent,
                    }),
                }
            }),
    )
    .padding(0)
    .style(move |theme: &Theme, _| button::Style {
        background: Some(theme.custom().transparent.into()),
        ..Default::default()
    });

    match navigate {
        Some(message) => button_widget.on_press(message).into(),
        None => button_widget.into(),
    }
}
