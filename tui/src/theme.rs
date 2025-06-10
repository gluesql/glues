use crate::color::*;
use ratatui::style::Color;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct Theme {
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub text_secondary: Color,
    pub accent: Color,
    pub accent_text: Color,
    pub warning: Color,
    pub warning_text: Color,
    pub success: Color,
    pub success_text: Color,
    pub error: Color,
    pub error_text: Color,
    pub dim: Color,
    pub medium: Color,
    pub light: Color,
    pub neutral_a: Color,
    pub neutral_b: Color,
    pub neutral_white: Color,
}

pub const DARK_THEME: Theme = Theme {
    background: GRAY_BLACK,
    surface: GRAY_DARK,
    text: WHITE,
    text_secondary: GRAY_LIGHT,
    accent: BLUE,
    accent_text: WHITE,
    warning: YELLOW,
    warning_text: BLACK,
    success: GREEN,
    success_text: BLACK,
    error: RED,
    error_text: WHITE,
    dim: GRAY_DIM,
    medium: GRAY_MEDIUM,
    light: GRAY_LIGHT,
    neutral_a: GRAY_A,
    neutral_b: GRAY_B,
    neutral_white: GRAY_WHITE,
};

#[allow(dead_code)]
pub const LIGHT_THEME: Theme = Theme {
    background: WHITE,
    surface: GRAY_WHITE,
    text: BLACK,
    text_secondary: GRAY_MEDIUM,
    accent: BLUE,
    accent_text: WHITE,
    warning: YELLOW,
    warning_text: BLACK,
    success: GREEN,
    success_text: BLACK,
    error: RED,
    error_text: WHITE,
    dim: GRAY_DIM,
    medium: GRAY_MEDIUM,
    light: GRAY_LIGHT,
    neutral_a: GRAY_A,
    neutral_b: GRAY_B,
    neutral_white: GRAY_WHITE,
};

pub static THEME: Theme = DARK_THEME;
