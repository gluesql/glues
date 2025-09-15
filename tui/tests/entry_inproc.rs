mod tester;
use tester::Tester;

use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;

#[tokio::test]
async fn entry_nav_enter_opens_instant_inproc() -> Result<()> {
    let mut t = Tester::new().await?;

    // initial draw (home)
    t.draw()?;

    // move down then up, then Enter to open Instant via selection
    t.press('j').await;
    t.press('k').await;
    t.key(KeyCode::Enter).await;

    // draw and assert notebook content is visible (e.g., sample note)
    t.draw()?;
    t.assert_contains("Sample Note");

    Ok(())
}

#[tokio::test]
async fn entry_quit_with_q_inproc() -> Result<()> {
    let mut t = Tester::new().await?;
    t.draw()?;

    let quit = t.press('q').await;
    assert!(quit);

    Ok(())
}

#[tokio::test]
async fn entry_help_overlay_open_close_inproc() -> Result<()> {
    let mut t = Tester::new().await?;
    t.draw()?;

    // open help (currently bound to 'a')
    t.press('a').await;
    t.draw()?;
    t.assert_contains("Press any key to close");

    // any key closes help
    t.press('x').await;
    t.draw()?;
    t.assert_not_contains("Press any key to close");

    Ok(())
}
