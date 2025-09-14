use color_eyre::Result;
use insta::assert_debug_snapshot;
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

#[tokio::test]
async fn home_to_instant_quit_inproc() -> Result<()> {
    // ensure logger/config have a writable HOME directory
    let cwd = std::env::current_dir()?;
    unsafe {
        std::env::set_var("HOME", &cwd);
    }
    config::init().await;
    logger::init().await;

    let backend = TestBackend::new(120, 40);
    let mut term = Terminal::new(backend)?;
    let mut app = App::new();

    // initial home screen
    app.draw_once_on(&mut term)?;
    assert_debug_snapshot!("home_inproc", buffer_to_lines(&term));

    // open Instant (in-memory) notebook
    app.handle_input(Input::Key(KeyEvent::new(
        KeyCode::Char('1'),
        KeyModifiers::NONE,
    )))
    .await;
    app.draw_once_on(&mut term)?;
    assert_debug_snapshot!("instant_inproc", buffer_to_lines(&term));

    // quit with Ctrl+C
    let quit = app
        .handle_input(Input::Key(KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL,
        )))
        .await;
    assert!(quit);

    Ok(())
}
