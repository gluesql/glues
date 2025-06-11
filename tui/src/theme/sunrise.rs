use super::Theme;
use ratatui::style::Color;

pub const SUNRISE_THEME: Theme = Theme {
    background: Color::Rgb(255, 249, 240),
    surface: Color::Rgb(255, 255, 254),
    panel: Color::Rgb(255, 235, 205),
    text: Color::Rgb(32, 32, 32),
    text_secondary: Color::Rgb(80, 70, 60),
    hint: Color::Rgb(128, 120, 100),
    menu: Color::Rgb(70, 60, 50),
    accent: Color::Rgb(255, 102, 0),
    accent_text: Color::Rgb(255, 255, 255),
    highlight: Color::Rgb(255, 140, 50),
    target: Color::Rgb(200, 90, 180),
    warning: Color::Rgb(255, 200, 0),
    warning_text: Color::Rgb(32, 32, 32),
    success: Color::Rgb(60, 175, 75),
    success_text: Color::Rgb(255, 255, 255),
    error: Color::Rgb(210, 50, 40),
    error_text: Color::Rgb(255, 255, 255),
    inactive_text: Color::Rgb(120, 120, 120),
    inactive_bg: Color::Rgb(240, 220, 200),
    crumb_a: Color::Rgb(250, 240, 230),
    crumb_b: Color::Rgb(240, 220, 210),
};
