use super::Theme;
use ratatui::style::Color;

pub const FOREST_THEME: Theme = Theme {
    background: Color::Rgb(15, 20, 15),
    surface: Color::Rgb(25, 30, 25),
    panel: Color::Rgb(40, 50, 40),
    text: Color::Rgb(220, 220, 215),
    text_secondary: Color::Rgb(160, 160, 155),
    hint: Color::Rgb(120, 130, 120),
    menu: Color::Rgb(140, 150, 140),
    accent: Color::Rgb(70, 160, 70),
    accent_text: Color::Rgb(0, 0, 0),
    highlight: Color::Rgb(100, 200, 100),
    target: Color::Rgb(200, 160, 70),
    warning: Color::Rgb(255, 180, 20),
    warning_text: Color::Rgb(32, 32, 32),
    success: Color::Rgb(100, 220, 120),
    success_text: Color::Rgb(0, 0, 0),
    error: Color::Rgb(200, 70, 50),
    error_text: Color::Rgb(0, 0, 0),
    inactive_text: Color::Rgb(100, 110, 100),
    inactive_bg: Color::Rgb(50, 60, 50),
    crumb_a: Color::Rgb(30, 40, 30),
    crumb_b: Color::Rgb(45, 55, 45),
};
