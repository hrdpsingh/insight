use crate::constant;
use iced::widget::{Svg, svg};

pub fn view<'a>(bytes: &'static [u8]) -> Svg<'a> {
    svg(svg::Handle::from_memory(bytes))
        .height(constant::icon::SIZE)
        .width(constant::icon::SIZE)
}
