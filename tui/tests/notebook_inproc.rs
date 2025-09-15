#![cfg(feature = "test-utils")]

use color_eyre::Result;
use ratatui::{
    Terminal,
    backend::TestBackend,
    crossterm::event::{Event as Input, KeyCode, KeyEvent, KeyModifiers},
};

use glues::{App, config, logger};

fn buffer_to_lines(term: &Terminal<TestBackend>) -> Vec<String> {
    let buf = term.backend().buffer().clone();
    let area = buf.area();
    let mut lines = Vec::with_capacity(area.height as usize);
    for y in 0..area.height {
        let mut line = String::new();
        for x in 0..area.width {
            line.push_str(buf[(x, y)].symbol());
        }
        lines.push(line);
    }
    lines
}

async fn setup() -> Result<(App, Terminal<TestBackend>)> {
    // ensure logger/config have a writable HOME directory
    let cwd = std::env::current_dir()?;
    unsafe {
        std::env::set_var("HOME", &cwd);
    }
    config::init().await;
    logger::init().await;

    let backend = TestBackend::new(120, 40);
    let term = Terminal::new(backend)?;
    let app = App::new();

    Ok((app, term))
}

async fn open_instant(app: &mut App, term: &mut Terminal<TestBackend>) -> Result<()> {
    app.draw_once_on(term)?;
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('1'),
        KeyModifiers::NONE,
    )))
    .await;
    app.draw_once_on(term)?;
    Ok(())
}

#[tokio::test]
async fn notebook_open_note_with_l_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;
    open_instant(&mut app, &mut term).await?;

    // select first note and open
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('j'),
        KeyModifiers::NONE,
    )))
    .await;
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('l'),
        KeyModifiers::NONE,
    )))
    .await;
    app.draw_once_on(&mut term)?;

    // editor shows sample content
    let buf = buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Hi :D"));

    Ok(())
}

#[tokio::test]
async fn notebook_note_actions_dialog_open_close_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;
    open_instant(&mut app, &mut term).await?;

    // select note to enable note actions
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('j'),
        KeyModifiers::NONE,
    )))
    .await;

    // open note actions dialog
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('m'),
        KeyModifiers::NONE,
    )))
    .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Note Actions"));

    // close dialog with Esc
    app.handle_input(Input::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)))
        .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(!buf.contains("Note Actions"));

    Ok(())
}

#[tokio::test]
async fn notebook_directory_actions_dialog_open_close_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;
    open_instant(&mut app, &mut term).await?;

    // on root directory selection, open directory actions dialog
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('m'),
        KeyModifiers::NONE,
    )))
    .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Directory Actions"));

    // close dialog with Esc
    app.handle_input(Input::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)))
        .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(!buf.contains("Directory Actions"));

    Ok(())
}

#[tokio::test]
async fn notebook_keymap_toggle_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;
    open_instant(&mut app, &mut term).await?;

    // show keymap
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('?'),
        KeyModifiers::NONE,
    )))
    .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(buf.contains(" [?] Hide keymap "));

    // hide keymap
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('?'),
        KeyModifiers::NONE,
    )))
    .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(!buf.contains(" [?] Hide keymap "));

    Ok(())
}

#[tokio::test]
async fn notebook_editor_keymap_in_insert_mode_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;
    open_instant(&mut app, &mut term).await?;

    // open note and enter insert mode
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('j'),
        KeyModifiers::NONE,
    )))
    .await;
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('l'),
        KeyModifiers::NONE,
    )))
    .await;
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('i'),
        KeyModifiers::NONE,
    )))
    .await;

    // show editor keymap with Ctrl+h
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('h'),
        KeyModifiers::CONTROL,
    )))
    .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Editor Keymap"));

    // any key closes
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('x'),
        KeyModifiers::NONE,
    )))
    .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(!buf.contains("Editor Keymap"));

    Ok(())
}

#[tokio::test]
async fn notebook_quit_confirm_then_cancel_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;
    open_instant(&mut app, &mut term).await?;

    // open note in normal mode (idle)
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('j'),
        KeyModifiers::NONE,
    )))
    .await;
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('l'),
        KeyModifiers::NONE,
    )))
    .await;

    // Esc opens quit confirmation
    app.handle_input(Input::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)))
        .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Do you want to quit?"));

    // cancel with 'n' (should not quit)
    let quit = app
        .handle_input(Input::Key(KeyEvent::new(
            KeyCode::Char('n'),
            KeyModifiers::NONE,
        )))
        .await;
    assert!(!quit);

    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(!buf.contains("Do you want to quit?"));

    Ok(())
}

#[tokio::test]
async fn notebook_quit_confirm_then_accept_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;
    open_instant(&mut app, &mut term).await?;

    // open note in normal mode (idle)
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('j'),
        KeyModifiers::NONE,
    )))
    .await;
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('l'),
        KeyModifiers::NONE,
    )))
    .await;

    // Esc then 'y' should quit
    app.handle_input(Input::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)))
        .await;
    let quit = app
        .handle_input(Input::Key(KeyEvent::new(
            KeyCode::Char('y'),
            KeyModifiers::NONE,
        )))
        .await;
    assert!(quit);

    Ok(())
}
