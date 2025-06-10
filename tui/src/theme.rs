use crate::color::*;
use ratatui::style::Color;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct Theme {
    pub background: Color,
    pub surface: Color,
    /// background for status bars or popups
    pub panel: Color,
    pub text: Color,
    pub text_secondary: Color,
    /// hint or description text
    pub hint: Color,
    /// menu item text
    pub menu: Color,
    pub accent: Color,
    pub accent_text: Color,
    pub highlight: Color,
    pub target: Color,
    pub warning: Color,
    pub warning_text: Color,
    pub success: Color,
    pub success_text: Color,
    pub error: Color,
    pub error_text: Color,
    /// text for disabled or unfocused elements
    pub inactive_text: Color,
    /// background for inactive highlights such as tabs
    pub inactive_bg: Color,
    /// alternating breadcrumb background colors
    pub crumb_a: Color,
    pub crumb_b: Color,
}

pub const DARK_THEME: Theme = Theme {
    background: GRAY_BLACK,
    surface: GRAY_DARK,
    panel: GRAY_WHITE,
    text: WHITE,
    text_secondary: GRAY_LIGHT,
    hint: GRAY_MEDIUM,
    menu: GRAY_LIGHT,
    accent: BLUE,
    accent_text: WHITE,
    highlight: SKY_BLUE,
    target: MAGENTA,
    warning: YELLOW,
    warning_text: BLACK,
    success: GREEN,
    success_text: BLACK,
    error: RED,
    error_text: WHITE,
    inactive_text: GRAY_DIM,
    inactive_bg: GRAY_DIM,
    crumb_a: GRAY_A,
    crumb_b: GRAY_B,
};

#[allow(dead_code)]
pub const LIGHT_THEME: Theme = Theme {
    background: WHITE,
    surface: GRAY_WHITE,
    panel: GRAY_LIGHT,
    text: BLACK,
    text_secondary: GRAY_MEDIUM,
    hint: GRAY_MEDIUM,
    menu: GRAY_DARK,
    accent: BLUE,
    accent_text: WHITE,
    highlight: BLUE,
    target: PINK,
    warning: YELLOW,
    warning_text: BLACK,
    success: GREEN,
    success_text: BLACK,
    error: RED,
    error_text: WHITE,
    inactive_text: GRAY_DARK,
    inactive_bg: GRAY_LIGHT,
    crumb_a: GRAY_B,
    crumb_b: GRAY_A,
};

pub static THEME: Theme = DARK_THEME;
