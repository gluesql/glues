use super::Theme;
use ratatui::style::Color;

pub const MIDNIGHT_THEME: Theme = Theme {
    background: Color::Rgb(10, 10, 20),
    surface: Color::Rgb(20, 20, 40),
    panel: Color::Rgb(40, 40, 60),
    text: Color::Rgb(220, 220, 230),
    text_secondary: Color::Rgb(150, 150, 170),
    hint: Color::Rgb(120, 120, 140),
    menu: Color::Rgb(100, 100, 120),
    accent: Color::Rgb(50, 150, 250),
    accent_text: Color::Rgb(0, 0, 0),
    highlight: Color::Rgb(70, 170, 255),
    target: Color::Rgb(200, 60, 250),
    warning: Color::Rgb(255, 190, 0),
    warning_text: Color::Rgb(0, 0, 0),
    success: Color::Rgb(40, 200, 40),
    success_text: Color::Rgb(0, 0, 0),
    error: Color::Rgb(240, 60, 60),
    error_text: Color::Rgb(0, 0, 0),
    inactive_text: Color::Rgb(90, 90, 110),
    inactive_bg: Color::Rgb(30, 30, 50),
    crumb_a: Color::Rgb(35, 35, 55),
    crumb_b: Color::Rgb(55, 55, 75),
    crumb_icon: Color::Rgb(255, 190, 0),
};
