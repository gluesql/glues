#![cfg(not(target_arch = "wasm32"))]

use {
    crate::{App, config, logger, theme},
    clap::{Args, Parser, ValueEnum},
    color_eyre::Result,
};

#[derive(Clone, Copy, ValueEnum)]
pub enum ThemeArg {
    Dark,
    Light,
    Pastel,
    Sunrise,
    Midnight,
    Forest,
}

#[derive(Clone, Default, Args)]
pub struct TuiArgs {
    #[arg(long, value_enum)]
    pub theme: Option<ThemeArg>,
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(flatten)]
    args: TuiArgs,
}

pub fn parse_args() -> TuiArgs {
    Cli::parse().args
}

impl From<ThemeArg> for theme::ThemeId {
    fn from(arg: ThemeArg) -> Self {
        match arg {
            ThemeArg::Dark => theme::ThemeId::Dark,
            ThemeArg::Light => theme::ThemeId::Light,
            ThemeArg::Pastel => theme::ThemeId::Pastel,
            ThemeArg::Sunrise => theme::ThemeId::Sunrise,
            ThemeArg::Midnight => theme::ThemeId::Midnight,
            ThemeArg::Forest => theme::ThemeId::Forest,
        }
    }
}

pub async fn run(args: TuiArgs) -> Result<()> {
    config::init().await;

    let saved_theme = config::get(config::LAST_THEME)
        .await
        .and_then(|value| value.parse().ok());
    let theme_id = args
        .theme
        .map(Into::into)
        .or(saved_theme)
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
    run(parse_args()).await
}
