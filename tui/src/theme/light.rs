use super::Theme;
use crate::color::*;

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
    crumb_icon: YELLOW,
};
