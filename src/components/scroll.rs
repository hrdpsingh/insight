use iced::{
    Background, Color, Element,
    widget::{
        container, scrollable,
        scrollable::{Direction, Scrollbar},
    },
};

pub fn view<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    scrollable(container(content))
        .direction(Direction::Vertical(
            Scrollbar::default().width(6).scroller_width(6),
        ))
        .style(|theme, status| {
            let mut base_style = scrollable::default(theme, status);

            match status {
                scrollable::Status::Hovered {
                    is_vertical_scrollbar_hovered,
                    ..
                } => {
                    base_style.vertical_rail.scroller.background =
                        Background::Color(if is_vertical_scrollbar_hovered {
                            Color::from_rgb8(160, 160, 160)
                        } else {
                            Color::from_rgb8(180, 180, 180)
                        });
                }
                scrollable::Status::Dragged { .. } => {
                    base_style.vertical_rail.scroller.background =
                        Background::Color(Color::from_rgb8(160, 160, 160));
                }
                scrollable::Status::Active { .. } => {
                    base_style.vertical_rail.scroller.background =
                        Background::Color(Color::from_rgb8(180, 180, 180));
                }
            }

            base_style
        })
        .into()
}
