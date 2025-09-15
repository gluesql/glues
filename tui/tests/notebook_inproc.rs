mod common;
use common::{AppTestExt as _, TerminalTestExt as _};

use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;

#[tokio::test]
async fn notebook_open_note_with_l_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    app.open_instant(&mut term).await?;

    // select first note and open
    app.press('j').await;
    app.press('l').await;
    app.draw_frame(&mut term)?;

    // editor shows sample content
    term.assert_contains("Hi :D");

    Ok(())
}

#[tokio::test]
async fn notebook_note_actions_dialog_open_close_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    app.open_instant(&mut term).await?;

    // select note to enable note actions
    app.press('j').await;

    // open note actions dialog
    app.press('m').await;
    app.draw_frame(&mut term)?;
    term.assert_contains("Note Actions");

    // close dialog with Esc
    app.key(KeyCode::Esc).await;
    app.draw_frame(&mut term)?;
    term.assert_not_contains("Note Actions");

    Ok(())
}

#[tokio::test]
async fn notebook_directory_actions_dialog_open_close_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    app.open_instant(&mut term).await?;

    // on root directory selection, open directory actions dialog
    app.press('m').await;
    app.draw_frame(&mut term)?;
    term.assert_contains("Directory Actions");

    // close dialog with Esc
    app.key(KeyCode::Esc).await;
    app.draw_frame(&mut term)?;
    term.assert_not_contains("Directory Actions");

    Ok(())
}

#[tokio::test]
async fn notebook_keymap_toggle_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    app.open_instant(&mut term).await?;

    // show keymap
    app.press('?').await;
    app.draw_frame(&mut term)?;
    term.assert_contains(" [?] Hide keymap ");

    // hide keymap
    app.press('?').await;
    app.draw_frame(&mut term)?;
    term.assert_not_contains("Do you want to quit?");

    Ok(())
}

#[tokio::test]
async fn notebook_quit_confirm_then_accept_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    app.open_instant(&mut term).await?;

    // open note in normal mode (idle)
    app.press('j').await;
    app.press('l').await;

    // Esc then 'y' should quit
    app.key(KeyCode::Esc).await;
    let quit = app.press('y').await;
    assert!(quit);

    Ok(())
}
