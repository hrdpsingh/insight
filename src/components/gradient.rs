use iced::{
    Color,
    gradient::{self, Gradient},
};

pub fn view(base: Color, intensity: f32) -> Gradient {
    let lightener = 1.0 + intensity;
    let darkener = 1.0 - intensity;

    Gradient::Linear(
        gradient::Linear::new(0)
            .add_stop(
                0.0,
                Color {
                    r: (base.r * darkener).clamp(0.0, 1.0),
                    g: (base.g * darkener).clamp(0.0, 1.0),
                    b: (base.b * darkener).clamp(0.0, 1.0),
                    a: base.a,
                },
            )
            .add_stop(0.5, base)
            .add_stop(
                1.0,
                Color {
                    r: (base.r * lightener).clamp(0.0, 1.0),
                    g: (base.g * lightener).clamp(0.0, 1.0),
                    b: (base.b * lightener).clamp(0.0, 1.0),
                    a: base.a,
                },
            ),
    )
}
