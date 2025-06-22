use once_cell::sync::Lazy;
use ratatui::style::Color;
use std::sync::RwLock;

pub mod dark;
pub mod forest;
pub mod light;
pub mod midnight;
pub mod pastel;
pub mod sunrise;

pub use {
    dark::DARK_THEME, forest::FOREST_THEME, light::LIGHT_THEME, midnight::MIDNIGHT_THEME,
    pastel::PASTEL_THEME, sunrise::SUNRISE_THEME,
};

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
    /// symbol color for breadcrumbs and directory icons
    pub crumb_icon: Color,
}

static THEME_CELL: Lazy<RwLock<Theme>> = Lazy::new(|| RwLock::new(DARK_THEME));

pub fn current_theme() -> Theme {
    *THEME_CELL.read().expect("theme read lock poisoned")
}

pub fn set_theme(theme: Theme) {
    *THEME_CELL.write().expect("theme write lock poisoned") = theme;
}
