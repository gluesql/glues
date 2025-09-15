#![cfg(feature = "test-utils")]

mod common;
use common::AppTestExt as _;

use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;

#[tokio::test]
async fn notebook_open_note_with_l_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    common::open_instant(&mut app, &mut term).await?;

    // select first note and open
    common::send_char(&mut app, 'j').await;
    common::send_char(&mut app, 'l').await;
    app.draw_frame(&mut term)?;

    // editor shows sample content
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Hi :D"));

    Ok(())
}

#[tokio::test]
async fn notebook_note_actions_dialog_open_close_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    common::open_instant(&mut app, &mut term).await?;

    // select note to enable note actions
    common::send_char(&mut app, 'j').await;

    // open note actions dialog
    common::send_char(&mut app, 'm').await;
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Note Actions"));

    // close dialog with Esc
    common::send_code(&mut app, KeyCode::Esc).await;
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(!buf.contains("Note Actions"));

    Ok(())
}

#[tokio::test]
async fn notebook_directory_actions_dialog_open_close_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    common::open_instant(&mut app, &mut term).await?;

    // on root directory selection, open directory actions dialog
    common::send_char(&mut app, 'm').await;
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Directory Actions"));

    // close dialog with Esc
    common::send_code(&mut app, KeyCode::Esc).await;
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(!buf.contains("Directory Actions"));

    Ok(())
}

#[tokio::test]
async fn notebook_keymap_toggle_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    common::open_instant(&mut app, &mut term).await?;

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
async fn notebook_editor_keymap_in_insert_mode_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    common::open_instant(&mut app, &mut term).await?;

    // open note and enter insert mode
    common::send_char(&mut app, 'j').await;
    common::send_char(&mut app, 'l').await;
    common::send_char(&mut app, 'i').await;

    // show editor keymap with Ctrl+h
    common::send_ctrl(&mut app, 'h').await;
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Editor Keymap"));

    // any key closes
    common::send_char(&mut app, 'x').await;
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(!buf.contains("Editor Keymap"));

    Ok(())
}

#[tokio::test]
async fn notebook_quit_confirm_then_cancel_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    common::open_instant(&mut app, &mut term).await?;

    // open note in normal mode (idle)
    common::send_char(&mut app, 'j').await;
    common::send_char(&mut app, 'l').await;

    // Esc opens quit confirmation
    common::send_code(&mut app, KeyCode::Esc).await;
    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Do you want to quit?"));

    // cancel with 'n' (should not quit)
    let quit = common::send_char(&mut app, 'n').await;
    assert!(!quit);

    app.draw_frame(&mut term)?;
    let buf = common::buffer_to_lines(&term).join("\n");
    assert!(!buf.contains("Do you want to quit?"));

    Ok(())
}

#[tokio::test]
async fn notebook_quit_confirm_then_accept_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;
    common::open_instant(&mut app, &mut term).await?;

    // open note in normal mode (idle)
    common::send_char(&mut app, 'j').await;
    common::send_char(&mut app, 'l').await;

    // Esc then 'y' should quit
    common::send_code(&mut app, KeyCode::Esc).await;
    let quit = common::send_char(&mut app, 'y').await;
    assert!(quit);

    Ok(())
}
