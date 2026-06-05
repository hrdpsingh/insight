pub mod button;
use crate::{app::Message, models::Page};
use iced::{
    Color, Element, Length, Shadow, Vector,
    widget::{column, container},
};

pub fn view<'a>(page: Page) -> Element<'a, Message> {
    container(
        column![button::view(
            include_bytes!("../../src/icons/processor.svg"),
            "Processor",
            Page::Processor,
            page
        ),]
        .spacing(20),
    )
    .width(140)
    .height(Length::Fill)
    .padding(10)
    .style(|theme| {
        let mut style = container::transparent(theme);
        style.background = Some(Color::from_rgb8(255, 255, 255).into());
        style.shadow = Shadow {
            color: Color::from_rgb8(160, 160, 160),
            offset: Vector::new(1.0, 0.0),
            blur_radius: 5.0,
        };
        style
    })
    .into()
}
