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
        background: Color::from_rgb(0.91, 0.95, 0.97),
        surface: Color::from_rgb(0.95, 0.98, 0.99),
        elevated: Color::from_rgb(0.96, 0.99, 1.0),
        accent: Color::from_rgb(0.3, 0.45, 0.8),
        accent_light: Color::from_rgb(0.7, 0.85, 1.0),
        text: Color::from_rgb(0.00, 0.00, 0.00),
        faded: Color::from_rgb(0.39, 0.39, 0.39),
        disabled: Color::from_rgb(0.67, 0.67, 0.67),
        shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
        scroller: Color::from_rgb(0.76, 0.78, 0.79),
        scroller_hovered: Color::from_rgb(0.73, 0.75, 0.76),
        transparent: Color::TRANSPARENT,
    };

    pub const DARK: Self = Self {
        background: Color::from_rgb(0.11, 0.12, 0.13),
        surface: Color::from_rgb(0.15, 0.16, 0.17),
        elevated: Color::from_rgb(0.19, 0.20, 0.21),
        accent: Color::from_rgb(0.3, 0.45, 0.8),
        accent_light: Color::from_rgb(0.7, 0.85, 1.0),
        text: Color::from_rgb(1.00, 1.00, 1.00),
        faded: Color::from_rgb(0.6, 0.6, 0.6),
        disabled: Color::from_rgb(0.47, 0.47, 0.47),
        shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
        scroller: Color::from_rgb(0.21, 0.22, 0.23),
        scroller_hovered: Color::from_rgb(0.23, 0.24, 0.25),
        transparent: Color::TRANSPARENT,
    };
}
