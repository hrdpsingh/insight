use crate::{app::Message, state::Page};
use iced::{Border, Color, Element, Length, border::Radius, widget::button};

pub fn view(label: &str, target_page: Page, current_page: Page) -> Element<'_, Message> {
    let is_selected = target_page == current_page;

    button(label)
        .on_press(Message::Navigate(target_page))
        .padding(8)
        .width(Length::Fill)
        .style(move |_, status: button::Status| {
            if is_selected || status == button::Status::Hovered || status == button::Status::Pressed
            {
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
