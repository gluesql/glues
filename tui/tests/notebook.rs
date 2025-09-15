#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;

#[tokio::test]
async fn opens_note_on_l() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // select first note and open
    t.press('j').await;
    t.press('l').await;
    t.draw()?;

    // editor shows sample content
    snap!(t, "note_open");

    Ok(())
}

#[tokio::test]
async fn note_actions_dialog_toggles() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // select note to enable note actions
    t.press('j').await;

    // open note actions dialog
    t.press('m').await;
    t.draw()?;
    snap!(t, "note_actions_open");

    // close dialog with Esc
    t.key(KeyCode::Esc).await;
    t.draw()?;
    snap!(t, "note_actions_closed");

    Ok(())
}

#[tokio::test]
async fn dir_actions_dialog_toggles() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // on root directory selection, open directory actions dialog
    t.press('m').await;
    t.draw()?;
    snap!(t, "dir_actions_open");

    // close dialog with Esc
    t.key(KeyCode::Esc).await;
    t.draw()?;
    snap!(t, "dir_actions_closed");

    Ok(())
}

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

#[tokio::test]
async fn quits_on_esc_then_y() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // open note in normal mode (idle)
    t.press('j').await;
    t.press('l').await;

    // Esc then 'y' should quit
    t.key(KeyCode::Esc).await;
    let quit = t.press('y').await;
    assert!(quit);

    Ok(())
}
