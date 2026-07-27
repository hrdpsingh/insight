pub mod process {
    pub const COUNT: usize = 10;
}

pub mod graph {
    pub const HISTORY_LENGTH: usize = 60;
}

pub mod border {
    pub const WIDTH: u32 = 1;
    pub const RADIUS: f32 = 12.0;
}

pub mod icon {
    pub const SIZE: u32 = 20;
}

pub mod spacing {
    pub const SMALL: u32 = 8;
    pub const MEDIUM: u32 = 16;
    pub const LARGE: u32 = 24;
}

pub mod padding {
    pub const NONE: f32 = 0.0;
    pub const SMALL: f32 = 8.0;
    pub const MEDIUM: f32 = 16.0;
    pub const LARGE: f32 = 24.0;
}

pub mod card {
    pub const WIDTH: f32 = 340.0;
    pub const HEIGHT_SMALL: f32 = 240.0;
    pub const HEIGHT_MEDIUM: f32 = 360.0;
    pub const HEIGHT_LARGE: f32 = 624.0;
}

pub mod breakpoint {
    pub const NARROW: f32 = 720.0;
    pub const WIDE: f32 = 1140.0;
}

pub mod window {
    pub const INITIAL_WIDTH: f32 = 960.0;
    pub const INITIAL_HEIGHT: f32 = 540.0;
    pub const MINIMUM_WIDTH: f32 = 270.0;
    pub const MINIMUM_HEIGHT: f32 = 480.0;
}
