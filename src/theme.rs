use iced::widget::{button, text_input};
use iced::{Border, Color, Theme};
use iced::theme::Palette;

pub fn round_button(theme: &Theme, status: button::Status) -> button::Style {
    let base = button::primary(theme, status);
    
    button::Style {
        border: Border {
            radius: 60.0.into(),
            ..base.border
        },
        ..base
    }
}

pub fn round_text_input(theme: &Theme, status: text_input::Status) -> text_input::Style {
    let base = text_input::default(theme, status);

    text_input::Style {
        border: Border {
            radius: 8.0.into(),
            ..base.border
        },
        ..base
    }
}


pub fn dark_theme() -> Theme {
    Theme::custom(
        "Dark Theme".to_string(),
        Palette {
        background: Color::from_rgb8(0x1a, 0x18, 0x22),
        text: Color::from_rgb8(0xdc, 0xd2, 0xf0),
        primary: Color::from_rgb8(0x9d, 0x5c, 0xf5),
        success: Color::from_rgb8(0x6c, 0xd6, 0x8a),
        warning: Color::from_rgb8(0xe8, 0xb3, 0x4d),
        danger: Color::from_rgb8(0xf5, 0x6c, 0x8a),
        },
    )
}