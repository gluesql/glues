#![cfg(feature = "test-utils")]

mod common;
use common::AppTestExt as _;

use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;

#[tokio::test]
async fn entry_nav_enter_opens_instant_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;

    // initial draw (home)
    app.draw_frame(&mut term)?;

    // move down then up, then Enter to open Instant via selection
    common::send_char(&mut app, 'j').await;
    common::send_char(&mut app, 'k').await;
    common::send_code(&mut app, KeyCode::Enter).await;

    // draw and assert notebook content is visible (e.g., sample note)
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Sample Note"));

    Ok(())
}

#[tokio::test]
async fn entry_quit_with_q_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    app.draw_frame(&mut term)?;

    let quit = common::send_char(&mut app, 'q').await;
    assert!(quit);

    Ok(())
}

#[tokio::test]
async fn entry_keymap_toggle_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    app.draw_frame(&mut term)?;

    // show keymap
    common::send_char(&mut app, '?').await;
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(buf.contains(" [?] Hide keymap "));

    // hide keymap
    common::send_char(&mut app, '?').await;
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(!buf.contains(" [?] Hide keymap "));

    Ok(())
}

#[tokio::test]
async fn entry_help_overlay_open_close_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    app.draw_frame(&mut term)?;

    // open help (currently bound to 'a')
    common::send_char(&mut app, 'a').await;
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Press any key to close"));

    // any key closes help
    common::send_char(&mut app, 'x').await;
    app.draw_once_on(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(!buf.contains("Press any key to close"));

    Ok(())
}
