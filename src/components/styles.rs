use iced::{
    Border, Color, Theme,
    border::Radius,
    widget::{button, container},
};

pub fn sidebar(_theme: &Theme) -> container::Style {
    container::Style::default()
        .background(Color::from_rgb8(255, 255, 255))
        .color(Color::from_rgb(0.0, 0.0, 0.0))
}

pub fn sidebar_button(_theme: &Theme, status: button::Status, selected: bool) -> button::Style {
    if selected || status == button::Status::Hovered || status == button::Status::Pressed {
        button::Style {
            background: Some(Color::from_rgb8(235, 245, 255).into()),
            text_color: Color::from_rgb8(0, 0, 0),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(5.0),
            },
            ..button::Style::default()
        }
    } else {
        button::Style {
            background: Some(Color::from_rgb8(255, 255, 255).into()),
            text_color: Color::from_rgb8(0, 0, 0),
            border: Border {
                radius: Radius::from(5.0),
                ..Border::default()
            },
            ..button::Style::default()
        }
    }
}
