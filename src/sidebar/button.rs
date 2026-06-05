use crate::{app::Message, models::Page};
use iced::{
    Alignment, Border, Color, Element, Length,
    border::Radius,
    widget::{button, row, svg},
};

pub fn view<'a>(
    icon_bytes: &'static [u8],
    label: &'a str,
    target_page: Page,
    current_page: Page,
) -> Element<'a, Message> {
    let is_selected = target_page == current_page;

    button(
        row![
            svg(svg::Handle::from_memory(icon_bytes))
                .width(20.0)
                .height(20.0),
            label
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    )
    .on_press(Message::Navigate(target_page))
    .padding(8)
    .width(Length::Fill)
    .style(move |_, status: button::Status| {
        if is_selected || status == button::Status::Hovered || status == button::Status::Pressed {
            button::Style {
                background: Some(Color::from_rgb8(215, 235, 255).into()),
                text_color: Color::from_rgb8(0, 0, 0),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(15.0),
                },
                ..button::Style::default()
            }
        } else {
            button::Style {
                background: Some(Color::from_rgb8(255, 255, 255).into()),
                text_color: Color::from_rgb8(0, 0, 0),
                border: Border {
                    radius: Radius::from(15.0),
                    ..Border::default()
                },
                ..button::Style::default()
            }
        }
    })
    .into()
}
