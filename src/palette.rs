use iced::{Color, Theme};

pub struct Shared {
    pub accent: Color,
    pub accent_light: Color,
    pub transparent: Color,
}

impl Shared {
    pub const COMMON: Self = Self {
        accent: Color::from_rgb(0.3, 0.45, 0.8),
        accent_light: Color::from_rgb(0.87, 0.93, 1.0),
        transparent: Color::TRANSPARENT,
    };
}

pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub border: Color,
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
        background: Color::from_rgb(0.91, 0.95, 0.97),
        surface: Color::from_rgb(0.95, 0.98, 0.99),
        border: Color::from_rgb(0.85, 0.9, 0.92),
        text: Color::from_rgb(0.00, 0.00, 0.00),
        faded: Color::from_rgb(0.39, 0.39, 0.39),
        disabled: Color::from_rgb(0.67, 0.67, 0.67),
        scroller: Color::from_rgb(0.8, 0.82, 0.83),
        scroller_hovered: Color::from_rgb(0.75, 0.77, 0.79),
        accent: Shared::COMMON.accent,
        accent_light: Shared::COMMON.accent_light,
        transparent: Shared::COMMON.transparent,
    };

    pub const DARK: Self = Self {
        background: Color::from_rgb(0.11, 0.12, 0.13),
        surface: Color::from_rgb(0.15, 0.16, 0.17),
        border: Color::from_rgb(0.17, 0.20, 0.22),
        text: Color::from_rgb(1.00, 1.00, 1.00),
        faded: Color::from_rgb(0.6, 0.6, 0.6),
        disabled: Color::from_rgb(0.47, 0.47, 0.47),
        scroller: Color::from_rgb(0.21, 0.22, 0.23),
        scroller_hovered: Color::from_rgb(0.23, 0.24, 0.25),
        accent: Shared::COMMON.accent,
        accent_light: Shared::COMMON.accent_light,
        transparent: Shared::COMMON.transparent,
    };
}
