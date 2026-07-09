use crate::{app::Message, palette::Palette, state::ExtendTheme};
use iced::{
    Background, Border, Color, Element, Length, Padding, Renderer, Shadow, Theme, Vector, gradient,
    widget::container,
};

fn card_bottom_multiplier(theme: &Theme) -> f32 {
    match theme {
        Theme::Dark => 0.92,
        _ => 0.97,
    }
}

fn border_top_multiplier(theme: &Theme) -> f32 {
    match theme {
        Theme::Dark => 1.1,
        _ => 1.01,
    }
}

fn border_bottom_multiplier(theme: &Theme) -> f32 {
    match theme {
        Theme::Dark => 0.8,
        _ => 0.95,
    }
}

pub fn view<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    width: Length,
    card_color: fn(&Palette) -> Color,
    padding: Padding,
) -> Element<'a, Message, Theme, Renderer> {
    container(
        container(
            container(content)
                .style(move |theme| {
                    let base_color = card_color(theme.custom());
                    let multiplier = card_bottom_multiplier(theme);

                    container::Style {
                        background: Some(Background::Gradient(
                            gradient::Linear::new(180_f32.to_radians())
                                .add_stop(0.0, base_color)
                                .add_stop(
                                    1.0,
                                    Color {
                                        r: base_color.r * multiplier,
                                        g: base_color.g * multiplier,
                                        b: base_color.b * multiplier,
                                        a: base_color.a,
                                    },
                                )
                                .into(),
                        )),
                        border: Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..container::Style::default()
                    }
                })
                .padding(padding)
                .width(Length::Fill)
                .height(Length::Shrink),
        )
        .style(move |theme| {
            let base_color = card_color(theme.custom());

            let top_multiplier = border_top_multiplier(theme);
            let bottom_multiplier = border_bottom_multiplier(theme);

            container::Style {
                background: Some(Background::Gradient(
                    gradient::Linear::new(180_f32.to_radians())
                        .add_stop(
                            0.0,
                            Color {
                                r: (base_color.r * top_multiplier).min(1.0),
                                g: (base_color.g * top_multiplier).min(1.0),
                                b: (base_color.b * top_multiplier).min(1.0),
                                a: base_color.a,
                            },
                        )
                        .add_stop(0.5, base_color)
                        .add_stop(
                            1.0,
                            Color {
                                r: base_color.r * bottom_multiplier,
                                g: base_color.g * bottom_multiplier,
                                b: base_color.b * bottom_multiplier,
                                a: base_color.a,
                            },
                        )
                        .into(),
                )),
                border: Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..container::Style::default()
            }
        })
        .padding(2),
    )
    .style(move |theme| container::Style {
        shadow: Shadow {
            color: theme.custom().shadow,
            offset: Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        border: Border {
            radius: 8.0.into(),
            ..Border::default()
        },
        ..container::Style::default()
    })
    .width(width)
    .height(Length::Shrink)
    .clip(true)
    .into()
}
