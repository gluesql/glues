use super::Theme;
use ratatui::style::Color;

pub const PASTEL_THEME: Theme = Theme {
    background: Color::Rgb(250, 247, 255),
    surface: Color::Rgb(255, 252, 255),
    panel: Color::Rgb(236, 232, 245),
    text: Color::Rgb(58, 58, 70),
    text_secondary: Color::Rgb(94, 94, 110),
    hint: Color::Rgb(132, 132, 150),
    menu: Color::Rgb(72, 72, 95),
    accent: Color::Rgb(142, 202, 230),
    accent_text: Color::Rgb(33, 49, 70),
    highlight: Color::Rgb(205, 180, 219),
    target: Color::Rgb(255, 175, 204),
    warning: Color::Rgb(255, 214, 165),
    warning_text: Color::Rgb(70, 60, 40),
    success: Color::Rgb(192, 242, 216),
    success_text: Color::Rgb(40, 70, 55),
    error: Color::Rgb(255, 173, 173),
    error_text: Color::Rgb(70, 40, 40),
    inactive_text: Color::Rgb(140, 140, 155),
    inactive_bg: Color::Rgb(228, 224, 235),
    crumb_a: Color::Rgb(244, 240, 248),
    crumb_b: Color::Rgb(232, 226, 240),
    crumb_icon: Color::Rgb(255, 200, 180),
};
