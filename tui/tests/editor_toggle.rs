#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;

#[tokio::test]
async fn toggle_mode_keymap() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // enter toggle mode and show keymap with ?
    t.press('t').await;
    t.press('?').await;
    t.draw()?;
    snap!(t, "toggle_mode_keymap");

    Ok(())
}

#[tokio::test]
async fn toggle_syntax_highlight() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // toggle syntax highlight off and back on without error
    t.press('t').await;
    t.press('s').await;
    t.press('t').await;
    t.press('s').await;
    t.draw()?;
    snap!(t, "after_syntax_toggle");

    Ok(())
}
