use crate::app::Message;
use iced::{
    Element,
    widget::{Column, Row},
};

pub fn masonry_layout<'a>(
    items: Vec<Element<'a, Message>>,
    columns: usize,
    spacing: f32,
) -> Element<'a, Message> {
    let mut cols: Vec<Vec<Element<'a, Message>>> = (0..columns).map(|_| Vec::new()).collect();

    for (i, item) in items.into_iter().enumerate() {
        cols[i % columns].push(item);
    }

    let column_widgets: Vec<Element<'a, Message>> = cols
        .into_iter()
        .map(|children| Column::with_children(children).spacing(spacing).into())
        .collect();

    Row::with_children(column_widgets).spacing(spacing).into()
}
