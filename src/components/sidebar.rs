use crate::{app::Message, components::button, models::Page};
use iced::{
    Color, Element, Shadow, Vector, border,
    widget::{column, container},
};

pub fn view<'a>(page: Page) -> Element<'a, Message> {
    container(
        column![
            button::view("Software", Page::Software, page),
            button::view("CPU", Page::Cpu, page),
            button::view("Firmware", Page::Firmware, page),
        ]
        .spacing(10),
    )
    .width(120)
    .padding(10)
    .style(|theme| {
        let mut style = container::transparent(theme);
        style.background = Some(Color::from_rgb8(255, 255, 255).into());
        style.border.radius = border::radius(15.0);
        style.shadow = Shadow {
            color: Color::from_rgb8(200, 200, 200),
            offset: Vector::new(1.0, 1.0),
            blur_radius: 5.0,
        };
        style
    })
    .into()
}
