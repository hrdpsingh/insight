use iced::{Color, Theme};

pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub elevated: Color,
    pub text: Color,
    pub faded: Color,
    pub disabled: Color,
    pub scroller: Color,
    pub scroller_hovered: Color,
    pub accent: Color,
    pub accent_light: Color,
    pub transparent: Color,
}

impl Palette {
    pub fn from(theme: &Theme) -> &'static Palette {
        match theme {
            Theme::Dark => &Palette::DARK,
            _ => &Palette::LIGHT,
        }
    }

    pub const LIGHT: Self = Self {
        background: Color::from_rgb(0.88, 0.92, 0.95),
        surface: Color::from_rgb(0.93, 0.96, 0.98),
        elevated: Color::from_rgb(0.96, 0.98, 1.0),
        text: Color::from_rgb(0.00, 0.00, 0.00),
        faded: Color::from_rgb(0.39, 0.39, 0.39),
        disabled: Color::from_rgb(0.67, 0.67, 0.67),
        scroller: Color::from_rgb(0.8, 0.82, 0.83),
        scroller_hovered: Color::from_rgb(0.75, 0.77, 0.79),
        accent: Color::from_rgb(0.3, 0.45, 0.8),
        accent_light: Color::from_rgb(0.87, 0.93, 1.0),
        transparent: Color::TRANSPARENT,
    };

    pub const DARK: Self = Self {
        background: Color::from_rgb(0.11, 0.12, 0.13),
        surface: Color::from_rgb(0.15, 0.16, 0.17),
        elevated: Color::from_rgb(0.17, 0.18, 0.19),
        text: Color::from_rgb(1.00, 1.00, 1.00),
        faded: Color::from_rgb(0.6, 0.6, 0.6),
        disabled: Color::from_rgb(0.47, 0.47, 0.47),
        scroller: Color::from_rgb(0.21, 0.22, 0.23),
        scroller_hovered: Color::from_rgb(0.23, 0.24, 0.25),
        accent: Color::from_rgb(0.25, 0.35, 0.6),
        accent_light: Color::from_rgb(0.4, 0.54, 0.71),
        transparent: Color::TRANSPARENT,
    };
}
