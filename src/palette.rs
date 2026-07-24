use iced::{Color, Theme};

pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub elevated: Color,
    pub text: Color,
    pub muted: Color,
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
        background: Color::from_rgb(0.84, 0.84, 0.84),
        surface: Color::from_rgb(0.92, 0.92, 0.92),
        elevated: Color::from_rgb(1.0, 1.0, 1.0),
        text: Color::from_rgb(0.00, 0.00, 0.00),
        muted: Color::from_rgb(0.35, 0.35, 0.35),
        scroller: Color::from_rgb(0.7, 0.7, 0.7),
        scroller_hovered: Color::from_rgb(0.65, 0.65, 0.65),
        accent: Color::from_rgb(0.4, 0.4, 0.8),
        accent_light: Color::from_rgba(0.8, 0.8, 1.0, 0.4),
        transparent: Color::TRANSPARENT,
    };

    pub const DARK: Self = Self {
        background: Color::from_rgb(0.0, 0.0, 0.0),
        surface: Color::from_rgb(0.08, 0.08, 0.08),
        elevated: Color::from_rgb(0.16, 0.16, 0.16),
        text: Color::from_rgb(1.00, 1.00, 1.00),
        muted: Color::from_rgb(0.65, 0.65, 0.65),
        scroller: Color::from_rgb(0.1, 0.1, 0.1),
        scroller_hovered: Color::from_rgb(0.15, 0.15, 0.15),
        accent: Color::from_rgb(0.4, 0.4, 0.8),
        accent_light: Color::from_rgba(0.8, 0.8, 1.0, 0.4),
        transparent: Color::TRANSPARENT,
    };
}
