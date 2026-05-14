use crate::{app::Message, components::styles, models::Page};
use iced::{
    Element, Length,
    widget::{button, column, container},
};

pub fn view<'a>(page: Page) -> Element<'a, Message> {
    container(
        column![
            button("Software")
                .on_press(Message::Navigate(Page::Software))
                .padding(8)
                .width(Length::Fill)
                .style(move |theme, status| {
                    styles::sidebar_button(theme, status, page == Page::Software)
                }),
            button("Hardware")
                .on_press(Message::Navigate(Page::Hardware))
                .padding(8)
                .width(Length::Fill)
                .style(move |theme, status| {
                    styles::sidebar_button(theme, status, page == Page::Hardware)
                }),
            button("Firmware")
                .on_press(Message::Navigate(Page::Firmware))
                .padding(8)
                .width(Length::Fill)
                .style(move |theme, status| {
                    styles::sidebar_button(theme, status, page == Page::Firmware)
                }),
        ]
        .spacing(10),
    )
    .width(120)
    .height(Length::Fill)
    .padding(10)
    .style(styles::sidebar)
    .into()
}
