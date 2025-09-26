use super::Theme;
use crate::color::*;

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
    inactive_bg: GRAY_MEDIUM,
    crumb_a: GRAY_A,
    crumb_b: GRAY_B,
    crumb_icon: YELLOW,
};
