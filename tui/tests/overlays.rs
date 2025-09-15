#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;

#[tokio::test]
async fn keymap_toggles() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // show keymap
    t.press('?').await;
    t.draw()?;
    snap!(t, "keymap_shown");

    // hide keymap
    t.press('?').await;
    t.draw()?;
    snap!(t, "keymap_hidden");

    Ok(())
}

