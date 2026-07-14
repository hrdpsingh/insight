use iced::{
    Background, Element,
    widget::{
        scrollable,
        scrollable::{Direction, Scrollbar},
    },
};

use crate::{components, palette::Palette};

pub fn view<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    scrollable(content)
        .direction(Direction::Vertical(
            Scrollbar::default().width(8).scroller_width(8).spacing(0),
        ))
        .style(|theme, status| {
            let mut base_style = scrollable::default(theme, status);
            base_style.vertical_rail.background =
                Some(Background::Color(Palette::from(theme).transparent));

            match status {
                scrollable::Status::Hovered {
                    is_vertical_scrollbar_hovered,
                    ..
                } => {
                    base_style.vertical_rail.scroller.background =
                        Background::Gradient(if is_vertical_scrollbar_hovered {
                            components::gradient::view(Palette::from(theme).scroller_hovered, 0.03)
                        } else {
                            components::gradient::view(Palette::from(theme).scroller, 0.03)
                        });
                }
                scrollable::Status::Dragged { .. } => {
                    base_style.vertical_rail.scroller.background = Background::Gradient(
                        components::gradient::view(Palette::from(theme).scroller_hovered, 0.03),
                    );
                }
                scrollable::Status::Active { .. } => {
                    base_style.vertical_rail.scroller.background = Background::Gradient(
                        components::gradient::view(Palette::from(theme).scroller, 0.03),
                    );
                }
            }

            base_style
        })
        .into()
}
