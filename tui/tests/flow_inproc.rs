#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;

#[tokio::test]
async fn home_to_instant_quit_inproc() -> Result<()> {
    let mut t = Tester::new().await?;

    // initial home screen
    t.draw()?;
    snap!(t, "home_inproc");

    // open Instant (in-memory) notebook
    t.open_instant().await?;
    snap!(t, "instant_inproc");

    // quit with Ctrl+C
    let quit = t.ctrl('c').await;
    assert!(quit);

    Ok(())
}
