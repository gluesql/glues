#![cfg(feature = "test-utils")]

mod common;

use color_eyre::Result;
use insta::assert_debug_snapshot;
use ratatui::crossterm::event::{Event as Input, KeyCode, KeyEvent, KeyModifiers};

#[tokio::test]
async fn home_to_instant_quit_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;

    // initial home screen
    app.draw_once_on(&mut term)?;
    assert_debug_snapshot!("home_inproc", common::buffer_to_lines(&term));

    // open Instant (in-memory) notebook
    common::open_instant(&mut app, &mut term).await?;
    assert_debug_snapshot!("instant_inproc", common::buffer_to_lines(&term));

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
