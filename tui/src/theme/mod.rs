use once_cell::sync::OnceCell;
use ratatui::style::Color;

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
