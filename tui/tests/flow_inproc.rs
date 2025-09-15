mod common;
use common::AppTestExt as _;

use color_eyre::Result;
use insta::assert_debug_snapshot;

#[tokio::test]
async fn home_to_instant_quit_inproc() -> Result<()> {
    let (mut app, mut term) = common::setup_app_and_term().await?;

    // initial home screen
    app.draw_frame(&mut term)?;
    assert_debug_snapshot!("home_inproc", common::buffer_to_lines(&term));

    // open Instant (in-memory) notebook
    app.open_instant(&mut term).await?;
    assert_debug_snapshot!("instant_inproc", common::buffer_to_lines(&term));

    // quit with Ctrl+C
    let quit = app.ctrl('c').await;
    assert!(quit);

    Ok(())
}
