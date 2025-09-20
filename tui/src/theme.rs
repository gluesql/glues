use {
    ratatui::style::Color,
    std::sync::atomic::{AtomicPtr, AtomicU8, Ordering},
};

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

const fn theme_ptr(theme: &'static Theme) -> *mut Theme {
    theme as *const Theme as *mut Theme
}

static CURRENT_THEME_PTR: AtomicPtr<Theme> = AtomicPtr::new(theme_ptr(&DARK_THEME));
static CURRENT_THEME_ID: AtomicU8 = AtomicU8::new(ThemeId::Dark as u8);

pub struct ThemeWrapper;

impl std::ops::Deref for ThemeWrapper {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        // SAFETY: the pointer always refers to one of the &'static Theme presets.
        unsafe { &*CURRENT_THEME_PTR.load(Ordering::Relaxed) }
    }
}

pub static THEME: ThemeWrapper = ThemeWrapper;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ThemeId {
    Dark = 0,
    Light = 1,
    Pastel = 2,
    Sunrise = 3,
    Midnight = 4,
    Forest = 5,
}

impl ThemeId {
    pub const fn as_str(self) -> &'static str {
        match self {
            ThemeId::Dark => "dark",
            ThemeId::Light => "light",
            ThemeId::Pastel => "pastel",
            ThemeId::Sunrise => "sunrise",
            ThemeId::Midnight => "midnight",
            ThemeId::Forest => "forest",
        }
    }
}

impl std::str::FromStr for ThemeId {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "dark" => Ok(ThemeId::Dark),
            "light" => Ok(ThemeId::Light),
            "pastel" => Ok(ThemeId::Pastel),
            "sunrise" => Ok(ThemeId::Sunrise),
            "midnight" => Ok(ThemeId::Midnight),
            "forest" => Ok(ThemeId::Forest),
            _ => Err(()),
        }
    }
}

pub struct ThemePreset {
    pub id: ThemeId,
    pub key: char,
    pub label: &'static str,
    pub theme: &'static Theme,
}

pub const PRESETS: [ThemePreset; 6] = [
    ThemePreset {
        id: ThemeId::Dark,
        key: 'd',
        label: "Dark",
        theme: &DARK_THEME,
    },
    ThemePreset {
        id: ThemeId::Light,
        key: 'l',
        label: "Light",
        theme: &LIGHT_THEME,
    },
    ThemePreset {
        id: ThemeId::Pastel,
        key: 'p',
        label: "Pastel",
        theme: &PASTEL_THEME,
    },
    ThemePreset {
        id: ThemeId::Sunrise,
        key: 's',
        label: "Sunrise",
        theme: &SUNRISE_THEME,
    },
    ThemePreset {
        id: ThemeId::Midnight,
        key: 'm',
        label: "Midnight",
        theme: &MIDNIGHT_THEME,
    },
    ThemePreset {
        id: ThemeId::Forest,
        key: 'f',
        label: "Forest",
        theme: &FOREST_THEME,
    },
];

pub fn preset_index(id: ThemeId) -> usize {
    id as usize
}

pub fn preset(id: ThemeId) -> &'static ThemePreset {
    &PRESETS[preset_index(id)]
}

pub fn preset_by_key(key: char) -> Option<&'static ThemePreset> {
    PRESETS.iter().find(|preset| preset.key == key)
}

pub fn set_theme(id: ThemeId) {
    let preset = preset(id);
    CURRENT_THEME_PTR.store(theme_ptr(preset.theme), Ordering::Relaxed);
    CURRENT_THEME_ID.store(id as u8, Ordering::Relaxed);
}

pub fn current_theme_id() -> ThemeId {
    match CURRENT_THEME_ID.load(Ordering::Relaxed) {
        0 => ThemeId::Dark,
        1 => ThemeId::Light,
        2 => ThemeId::Pastel,
        3 => ThemeId::Sunrise,
        4 => ThemeId::Midnight,
        5 => ThemeId::Forest,
        _ => ThemeId::Dark,
    }
}
