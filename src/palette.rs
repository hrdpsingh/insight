use iced::{Color, Theme};

pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub border: Color,
    pub text: Color,
    pub muted: Color,
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
        background: Color::from_rgb(0.95, 0.95, 0.95),
        surface: Color::from_rgb(1.0, 1.0, 1.0),
        border: Color::from_rgb(0.85, 0.85, 0.85),
        text: Color::from_rgb(0.00, 0.00, 0.00),
        muted: Color::from_rgb(0.35, 0.35, 0.35),
        disabled: Color::from_rgb(0.7, 0.7, 0.7),
        scroller: Color::from_rgb(0.7, 0.7, 0.7),
        scroller_hovered: Color::from_rgb(0.65, 0.65, 0.65),
        accent: Color::from_rgb(0.4, 0.4, 0.8),
        accent_light: Color::from_rgba(0.6, 0.6, 1.0, 0.5),
        transparent: Color::TRANSPARENT,
    };

    pub const DARK: Self = Self {
        background: Color::from_rgb(0.0, 0.0, 0.0),
        surface: Color::from_rgb(0.05, 0.05, 0.05),
        border: Color::from_rgb(0.15, 0.15, 0.15),
        text: Color::from_rgb(1.00, 1.00, 1.00),
        muted: Color::from_rgb(0.65, 0.65, 0.65),
        disabled: Color::from_rgb(0.3, 0.3, 0.3),
        scroller: Color::from_rgb(0.1, 0.1, 0.1),
        scroller_hovered: Color::from_rgb(0.15, 0.15, 0.15),
        accent: Color::from_rgb(0.4, 0.4, 0.8),
        accent_light: Color::from_rgba(0.6, 0.6, 1.0, 0.5),
        transparent: Color::TRANSPARENT,
    };
}
