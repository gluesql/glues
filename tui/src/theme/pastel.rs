use super::Theme;
use ratatui::style::Color;

pub const PASTEL_THEME: Theme = Theme {
    background: Color::Rgb(253, 253, 253),
    surface: Color::Rgb(255, 255, 255),
    panel: Color::Rgb(232, 232, 232),
    text: Color::Rgb(51, 51, 51),
    text_secondary: Color::Rgb(85, 85, 85),
    hint: Color::Rgb(136, 136, 136),
    menu: Color::Rgb(68, 68, 68),
    accent: Color::Rgb(0, 102, 204),
    accent_text: Color::Rgb(255, 255, 255),
    highlight: Color::Rgb(58, 158, 229),
    target: Color::Rgb(255, 105, 180),
    warning: Color::Rgb(255, 170, 0),
    warning_text: Color::Rgb(32, 32, 32),
    success: Color::Rgb(68, 204, 68),
    success_text: Color::Rgb(255, 255, 255),
    error: Color::Rgb(221, 51, 51),
    error_text: Color::Rgb(255, 255, 255),
    inactive_text: Color::Rgb(119, 119, 119),
    inactive_bg: Color::Rgb(221, 221, 221),
    crumb_a: Color::Rgb(240, 240, 240),
    crumb_b: Color::Rgb(224, 224, 224),
};
