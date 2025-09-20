use {crate::theme, ratatui::widgets::ListState};

pub struct ThemeSelector {
    pub list_state: ListState,
}

impl ThemeSelector {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        let current = theme::current_theme_id();
        let index = theme::preset_index(current);
        list_state.select(Some(index));
        Self { list_state }
    }

    pub fn presets(&self) -> &'static [theme::ThemePreset] {
        &theme::PRESETS
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    pub fn select_by_key(&mut self, key: char) -> Option<&'static theme::ThemePreset> {
        let preset = theme::preset_by_key(key)?;
        let index = theme::preset_index(preset.id);
        self.list_state.select(Some(index));
        Some(preset)
    }

    pub fn selected(&self) -> &'static theme::ThemePreset {
        let index = self
            .list_state
            .selected()
            .unwrap_or_else(|| theme::preset_index(theme::ThemeId::Dark));
        &theme::PRESETS[index]
    }
}

impl Default for ThemeSelector {
    fn default() -> Self {
        Self::new()
    }
}
