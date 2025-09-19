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

#[derive(Clone, Args)]
pub struct TuiArgs {
    #[arg(long, value_enum, default_value_t = ThemeArg::Dark)]
    pub theme: ThemeArg,
}

impl Default for TuiArgs {
    fn default() -> Self {
        Self {
            theme: ThemeArg::Dark,
        }
    }
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

pub async fn run(args: TuiArgs) -> Result<()> {
    match args.theme {
        ThemeArg::Dark => theme::set_theme(theme::DARK_THEME),
        ThemeArg::Light => theme::set_theme(theme::LIGHT_THEME),
        ThemeArg::Pastel => theme::set_theme(theme::PASTEL_THEME),
        ThemeArg::Sunrise => theme::set_theme(theme::SUNRISE_THEME),
        ThemeArg::Midnight => theme::set_theme(theme::MIDNIGHT_THEME),
        ThemeArg::Forest => theme::set_theme(theme::FOREST_THEME),
    }

    config::init().await;
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
