use iced::{
    Background, Color, Element,
    widget::{
        scrollable,
        scrollable::{Direction, Scrollbar},
    },
};

pub fn view<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    scrollable(content)
        .direction(Direction::Vertical(
            Scrollbar::default().width(8).scroller_width(8).spacing(0),
        ))
        .style(|theme, status| {
            let mut base_style = scrollable::default(theme, status);
            base_style.vertical_rail.background =
                Some(Background::Color(Color::from_rgb8(230, 230, 230)));

            match status {
                scrollable::Status::Hovered {
                    is_vertical_scrollbar_hovered,
                    ..
                } => {
                    base_style.vertical_rail.scroller.background =
                        Background::Color(if is_vertical_scrollbar_hovered {
                            Color::from_rgb8(200, 200, 200)
                        } else {
                            Color::from_rgb8(210, 210, 210)
                        });
                }
                scrollable::Status::Dragged { .. } => {
                    base_style.vertical_rail.scroller.background =
                        Background::Color(Color::from_rgb8(200, 200, 200));
                }
                scrollable::Status::Active { .. } => {
                    base_style.vertical_rail.scroller.background =
                        Background::Color(Color::from_rgb8(210, 210, 210));
                }
            }

            base_style
        })
        .into()
}
