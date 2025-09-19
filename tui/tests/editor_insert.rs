#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;
use glues_tui::input::KeyCode;

#[tokio::test]
async fn editor_keymap_overlay_in_insert_mode() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // open first note and enter insert mode
    t.open_first_note().await?;
    t.press('i').await;

    // show editor keymap (Ctrl+h)
    t.ctrl('h').await;
    t.draw()?;
    snap!(t, "editor_keymap_open");

    // any key closes overlay
    t.press('x').await;
    t.draw()?;
    snap!(t, "editor_keymap_closed");

    Ok(())
}

#[tokio::test]
async fn insert_typing_and_escape() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.press('i').await;
    t.draw()?;
    snap!(t, "insert_mode");

    t.type_str("Hello").await;
    t.draw()?;
    snap!(t, "inserted_text");

    // escape back to normal
    t.key(KeyCode::Esc).await;
    t.draw()?;
    snap!(t, "back_to_normal");

    Ok(())
}
