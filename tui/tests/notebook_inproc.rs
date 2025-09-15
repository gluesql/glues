mod common;
use common::Tester;

use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;

#[tokio::test]
async fn notebook_open_note_with_l_inproc() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // select first note and open
    t.press('j').await;
    t.press('l').await;
    t.draw()?;

    // editor shows sample content
    t.assert_contains("Hi :D");

    Ok(())
}

#[tokio::test]
async fn notebook_note_actions_dialog_open_close_inproc() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // select note to enable note actions
    t.press('j').await;

    // open note actions dialog
    t.press('m').await;
    t.draw()?;
    t.assert_contains("Note Actions");

    // close dialog with Esc
    t.key(KeyCode::Esc).await;
    t.draw()?;
    t.assert_not_contains("Note Actions");

    Ok(())
}

#[tokio::test]
async fn notebook_directory_actions_dialog_open_close_inproc() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // on root directory selection, open directory actions dialog
    t.press('m').await;
    t.draw()?;
    t.assert_contains("Directory Actions");

    // close dialog with Esc
    t.key(KeyCode::Esc).await;
    t.draw()?;
    t.assert_not_contains("Directory Actions");

    Ok(())
}

#[tokio::test]
async fn notebook_keymap_toggle_inproc() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // show keymap
    t.press('?').await;
    t.draw()?;
    t.assert_contains(" [?] Hide keymap ");

    // hide keymap
    t.press('?').await;
    t.draw()?;
    t.assert_not_contains("Do you want to quit?");

    Ok(())
}

#[tokio::test]
async fn notebook_quit_confirm_then_accept_inproc() -> Result<()> {
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
