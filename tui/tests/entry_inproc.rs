mod common;
use common::{AppTestExt as _, TerminalTestExt as _};

use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;

#[tokio::test]
async fn entry_nav_enter_opens_instant_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;

    // initial draw (home)
    app.draw_frame(&mut term)?;

    // move down then up, then Enter to open Instant via selection
    app.press('j').await;
    app.press('k').await;
    app.key(KeyCode::Enter).await;

    // draw and assert notebook content is visible (e.g., sample note)
    app.draw_frame(&mut term)?;
    term.assert_contains("Sample Note");

    Ok(())
}

#[tokio::test]
async fn entry_quit_with_q_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    app.draw_frame(&mut term)?;

    let quit = app.press('q').await;
    assert!(quit);

    Ok(())
}

#[tokio::test]
async fn entry_help_overlay_open_close_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    app.draw_frame(&mut term)?;

    // open help (currently bound to 'a')
    app.press('a').await;
    app.draw_frame(&mut term)?;
    term.assert_contains("Press any key to close");

    // any key closes help
    app.press('x').await;
    app.draw_frame(&mut term)?;
    term.assert_not_contains("Press any key to close");

    Ok(())
}
