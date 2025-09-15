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

#[tokio::test]
async fn entry_nav_enter_opens_instant_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;

    // initial draw (home)
    app.draw_once_on(&mut term)?;

    // move down then up, then Enter to open Instant via selection
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('j'),
        KeyModifiers::NONE,
    )))
    .await;
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('k'),
        KeyModifiers::NONE,
    )))
    .await;
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Enter,
        KeyModifiers::NONE,
    )))
    .await;

    // draw and assert notebook content is visible (e.g., sample note)
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Sample Note"));

    Ok(())
}

#[tokio::test]
async fn entry_quit_with_q_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;
    app.draw_once_on(&mut term)?;

    let quit = app
        .handle_input(Input::Key(KeyEvent::new(
            KeyCode::Char('q'),
            KeyModifiers::NONE,
        )))
        .await;
    assert!(quit);

    Ok(())
}

#[tokio::test]
async fn entry_keymap_toggle_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;
    app.draw_once_on(&mut term)?;

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
async fn entry_help_overlay_open_close_inproc() -> Result<()> {
    let (mut app, mut term) = setup().await?;
    app.draw_once_on(&mut term)?;

    // open help (currently bound to 'a')
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('a'),
        KeyModifiers::NONE,
    )))
    .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(buf.contains("Press any key to close"));

    // any key closes help
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('x'),
        KeyModifiers::NONE,
    )))
    .await;
    app.draw_once_on(&mut term)?;
    let buf = buffer_to_lines(&term).join("\n");
    assert!(!buf.contains("Press any key to close"));

    Ok(())
}
