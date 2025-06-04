use iced::{
    widget::button::{Style, Status},
    Color, Border, Shadow, Theme,
};
use iced::border::Radius;
pub fn red_close_button_style(_theme: &Theme, status: Status) -> Style {
    match status {
        Status::Disabled => Style {
            background: None,
            text_color: Color {
                r: 0.85,
                g: 0.85,
                b: 0.85,
                a: 1.0,
            },
            border: Border {
                color: Color::TRANSPARENT,
                radius: Radius::default(),
                width: 1.0,
            },
            shadow: Shadow::default(),
        },
        _ => Style {
            background: Some(Color { r: 0.85, g: 0.1, b: 0.1, a: 1.0 }.into()),
            text_color: Color::WHITE,
            border: Border {
                color: Color { r: 0.7, g: 0.0, b: 0.0, a: 1.0 },
                radius: Radius::new(3.0),
                width: 1.0,
            },
            shadow: Shadow {
                offset: iced::Vector::new(0.0, 1.0),
                color: Color { r: 0.6, g: 0.0, b: 0.0, a: 0.6 },
                blur_radius: 2.0,
            },
        },
    }
}