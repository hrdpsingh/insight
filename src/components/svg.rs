use crate::constant::ICON_SIZE;
use iced::widget::{Svg, svg};

pub fn view<'a>(bytes: &'static [u8]) -> Svg<'a> {
    svg(svg::Handle::from_memory(bytes))
        .height(ICON_SIZE)
        .width(ICON_SIZE)
}
