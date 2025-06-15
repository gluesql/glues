use {
    clap::{Parser, ValueEnum},
    color_eyre::Result,
    glues::logger::log,
    glues::{App, config, theme},
};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(long, value_enum, default_value_t = ThemeArg::Dark)]
    theme: ThemeArg,
}

#[derive(Clone, Copy, ValueEnum)]
enum ThemeArg {
    Dark,
    Light,
    Pastel,
    Sunrise,
    Midnight,
    Forest,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.theme {
        ThemeArg::Dark => theme::set_theme(theme::DARK_THEME),
        ThemeArg::Light => theme::set_theme(theme::LIGHT_THEME),
        ThemeArg::Pastel => theme::set_theme(theme::PASTEL_THEME),
        ThemeArg::Sunrise => theme::set_theme(theme::SUNRISE_THEME),
        ThemeArg::Midnight => theme::set_theme(theme::MIDNIGHT_THEME),
        ThemeArg::Forest => theme::set_theme(theme::FOREST_THEME),
    }

    config::init().await;
    glues::logger::init().await;
    color_eyre::install()?;

    glues::log!("Hello");

    let terminal = ratatui::init();
    let app_result = App::new().await.run(terminal).await;
    ratatui::restore();
    app_result
}
