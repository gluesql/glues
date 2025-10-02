#[macro_use]
mod tester;
use tester::Tester;

use {
    color_eyre::Result,
    glues_tui::{config::LAST_PROXY_URL, input::KeyCode},
};

#[tokio::test]
async fn opens_instant_on_enter() -> Result<()> {
    let mut t = Tester::new().await?;

    // initial draw (home)
    t.draw()?;

    // move down then up, then Enter to open Instant via selection
    t.press('j').await;
    t.press('k').await;
    t.key(KeyCode::Enter).await;

    // draw and snapshot notebook view (e.g., sample note visible)
    t.draw()?;
    snap!(t, "instant");

    Ok(())
}

#[tokio::test]
async fn quits_on_q() -> Result<()> {
    let mut t = Tester::new().await?;
    t.draw()?;

    let quit = t.press('q').await;
    assert!(quit);

    Ok(())
}

#[tokio::test]
async fn help_overlay_toggles() -> Result<()> {
    let mut t = Tester::new().await?;
    t.draw()?;

    // open help via shortcut
    t.press('h').await;
    t.draw()?;
    snap!(t, "help_open");

    // any key closes help
    t.press('x').await;
    t.draw()?;
    snap!(t, "help_closed");

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn open_local_prompt() -> Result<()> {
    let mut t = Tester::new().await?;

    t.draw()?;
    // open Local storage prompt via hotkey 'l'
    t.press('l').await;
    t.draw()?;
    snap!(t, "local_prompt");

    Ok(())
}

#[tokio::test]
async fn open_redb_prompt() -> Result<()> {
    let mut t = Tester::new().await?;

    t.draw()?;
    // open redb storage prompt via hotkey 'r'
    t.press('r').await;
    t.draw()?;
    snap!(t, "redb_prompt");

    Ok(())
}

#[tokio::test]
async fn proxy_prompt_open() -> Result<()> {
    let mut t = Tester::new().await?;
    glues_tui::config::update(LAST_PROXY_URL, "").await;
    t.draw()?;

    // open Proxy prompt via hotkey 'p'
    t.press('p').await;
    t.draw()?;
    snap!(t, "proxy_prompt_open");

    // proceed to the auth-token prompt
    t.type_str("http://127.0.0.1:9").await;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "proxy_token_prompt");

    // provide a token (masked) and expect a connection failure alert
    t.type_str("secret").await;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "proxy_connect_error");

    glues_tui::config::update(LAST_PROXY_URL, "").await;

    Ok(())
}

#[tokio::test]
async fn theme_dialog_open() -> Result<()> {
    let mut t = Tester::new().await?;

    t.draw()?;
    t.press('t').await;
    t.draw()?;
    snap!(t, "theme_dialog_open");

    Ok(())
}

#[tokio::test]
async fn config_isolation_test() -> Result<()> {
    use std::path::PathBuf;

    let home_config = PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".glues");
    let home_config_existed_before = home_config.exists();

    let _t = Tester::new().await?;

    glues_tui::config::update("last_csv_path", "/test/path").await;
    let value = glues_tui::config::get("last_csv_path").await;
    assert_eq!(value, Some("/test/path".to_string()));

    let home_config_exists_after = home_config.exists();
    assert_eq!(
        home_config_existed_before, home_config_exists_after,
        "Test should not create .glues in user's home directory"
    );

    if let Ok(config_dir) = std::env::var("GLUES_CONFIG_DIR") {
        let test_config = PathBuf::from(config_dir);
        assert!(
            test_config.exists(),
            "Test config directory should exist at GLUES_CONFIG_DIR"
        );
        assert_ne!(
            test_config, home_config,
            "Test config should not be in user's home directory"
        );
    }

    Ok(())
}
