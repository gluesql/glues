#![cfg(not(target_arch = "wasm32"))]

use {
    crate::{App, config, logger, theme},
    color_eyre::Result,
};

pub async fn run() -> Result<()> {
    config::init().await;

    let theme_id = config::get(config::LAST_THEME)
        .await
        .and_then(|value| value.parse().ok())
        .unwrap_or(theme::ThemeId::Dark);

    theme::set_theme(theme_id);
    config::update(config::LAST_THEME, theme_id.as_str()).await;

    logger::init().await;
    color_eyre::install()?;

    let terminal = ratatui::init();
    let app_result = App::new().run(terminal).await;
    ratatui::restore();
    app_result
}

pub async fn run_cli() -> Result<()> {
    run().await
}
