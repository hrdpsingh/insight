use iced::Color;

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub elevated: Color,
    pub accent: Color,
    pub accent_light: Color,
    pub text: Color,
    pub faded: Color,
    pub disabled: Color,
    pub shadow: Color,
    pub scroller: Color,
    pub scroller_hovered: Color,
    pub transparent: Color,
}

impl Palette {
    pub const LIGHT: Self = Self {
        background: Color::from_rgb(0.96, 0.96, 0.96),
        surface: Color::from_rgb(0.98, 0.98, 0.98),
        elevated: Color::from_rgb(1.00, 1.00, 1.00),
        accent: Color::from_rgb(0.3, 0.45, 0.8),
        accent_light: Color::from_rgb(0.7, 0.85, 1.0),
        text: Color::from_rgb(0.00, 0.00, 0.00),
        faded: Color::from_rgb(0.39, 0.39, 0.39),
        disabled: Color::from_rgb(0.67, 0.67, 0.67),
        shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
        scroller: Color::from_rgb(0.78, 0.78, 0.78),
        scroller_hovered: Color::from_rgb(0.75, 0.75, 0.75),
        transparent: Color::TRANSPARENT,
    };

    pub const DARK: Self = Self {
        background: Color::from_rgb(0.12, 0.12, 0.12),
        surface: Color::from_rgb(0.16, 0.16, 0.16),
        elevated: Color::from_rgb(0.20, 0.20, 0.20),
        accent: Color::from_rgb(0.3, 0.45, 0.8),
        accent_light: Color::from_rgb(0.7, 0.85, 1.0),
        text: Color::from_rgb(1.00, 1.00, 1.00),
        faded: Color::from_rgb(0.78, 0.78, 0.78),
        disabled: Color::from_rgb(0.47, 0.47, 0.47),
        shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
        scroller: Color::from_rgb(0.12, 0.12, 0.12),
        scroller_hovered: Color::from_rgb(0.35, 0.35, 0.35),
        transparent: Color::TRANSPARENT,
    };
}
