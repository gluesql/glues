use crate::color::*;
use once_cell::sync::OnceCell;
use ratatui::style::Color;
use serde::Deserialize;
use std::path::Path;

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

#[derive(Deserialize)]
struct ThemeToml {
    background: String,
    surface: String,
    panel: String,
    text: String,
    text_secondary: String,
    hint: String,
    menu: String,
    accent: String,
    accent_text: String,
    highlight: String,
    target: String,
    warning: String,
    warning_text: String,
    success: String,
    success_text: String,
    error: String,
    error_text: String,
    inactive_text: String,
    inactive_bg: String,
    crumb_a: String,
    crumb_b: String,
}

fn parse_color(value: &str) -> Option<Color> {
    let hex = value.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}

pub fn load_theme_from_path(path: &Path) -> Result<Theme, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let value: ThemeToml = toml::from_str(&content)?;

    macro_rules! parse {
        ($field:ident) => {
            parse_color(&value.$field)
                .ok_or_else(|| format!("invalid color for {}", stringify!($field)))?
        };
    }

    Ok(Theme {
        background: parse!(background),
        surface: parse!(surface),
        panel: parse!(panel),
        text: parse!(text),
        text_secondary: parse!(text_secondary),
        hint: parse!(hint),
        menu: parse!(menu),
        accent: parse!(accent),
        accent_text: parse!(accent_text),
        highlight: parse!(highlight),
        target: parse!(target),
        warning: parse!(warning),
        warning_text: parse!(warning_text),
        success: parse!(success),
        success_text: parse!(success_text),
        error: parse!(error),
        error_text: parse!(error_text),
        inactive_text: parse!(inactive_text),
        inactive_bg: parse!(inactive_bg),
        crumb_a: parse!(crumb_a),
        crumb_b: parse!(crumb_b),
    })
}

pub struct ThemeWrapper;

static THEME_CELL: OnceCell<Theme> = OnceCell::new();

impl std::ops::Deref for ThemeWrapper {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        THEME_CELL.get_or_init(|| DARK_THEME)
    }
}

pub static THEME: ThemeWrapper = ThemeWrapper;

pub fn set_theme(theme: Theme) {
    let _ = THEME_CELL.set(theme);
}
