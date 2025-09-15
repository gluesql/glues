mod common;
use common::{AppTestExt as _, TerminalTestExt as _};

use color_eyre::Result;

#[tokio::test]
async fn home_to_instant_quit_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;

    // initial home screen
    app.draw_frame(&mut term)?;
    term.assert_snapshot("home_inproc");

    // open Instant (in-memory) notebook
    app.open_instant(&mut term).await?;
    term.assert_snapshot("instant_inproc");

    // quit with Ctrl+C
    let quit = app.ctrl('c').await;
    assert!(quit);

    Ok(())
}
