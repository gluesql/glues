#[macro_use]
mod tester;
use tester::Tester;

use {color_eyre::Result, glues_tui::input::KeyCode};

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
async fn proxy_prompt_open() -> Result<()> {
    let mut t = Tester::new().await?;
    t.draw()?;

    // open Proxy prompt via hotkey 'p'
    t.press('p').await;
    t.draw()?;
    snap!(t, "proxy_prompt_open");

    Ok(())
}
