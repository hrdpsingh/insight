use iced::{
    Background, Element,
    widget::{
        scrollable,
        scrollable::{Direction, Scrollbar},
    },
};

use crate::palette;

pub fn view<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    scrollable(content)
        .direction(Direction::Vertical(
            Scrollbar::default().width(8).scroller_width(8).spacing(0),
        ))
        .style(|theme, status| {
            let mut base_style = scrollable::default(theme, status);
            base_style.vertical_rail.background =
                Some(Background::Color(palette::SCROLLBAR_BACKGROUND));

            match status {
                scrollable::Status::Hovered {
                    is_vertical_scrollbar_hovered,
                    ..
                } => {
                    base_style.vertical_rail.scroller.background =
                        Background::Color(if is_vertical_scrollbar_hovered {
                            palette::SCROLLER_HOVERED
                        } else {
                            palette::SCROLLER
                        });
                }
                scrollable::Status::Dragged { .. } => {
                    base_style.vertical_rail.scroller.background =
                        Background::Color(palette::SCROLLER_HOVERED);
                }
                scrollable::Status::Active { .. } => {
                    base_style.vertical_rail.scroller.background =
                        Background::Color(palette::SCROLLER);
                }
            }

            base_style
        })
        .into()
}
