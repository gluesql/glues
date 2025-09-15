mod tester;
use tester::Tester;

use color_eyre::Result;

#[tokio::test]
async fn home_to_instant_quit_inproc() -> Result<()> {
    let mut t = Tester::new().await?;

    // initial home screen
    t.draw()?;
    t.assert_snapshot("home_inproc");

    // open Instant (in-memory) notebook
    t.open_instant().await?;
    t.assert_snapshot("instant_inproc");

    // quit with Ctrl+C
    let quit = t.ctrl('c').await;
    assert!(quit);

    Ok(())
}
