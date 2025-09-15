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
async fn add_note_via_directory_actions() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // open directory actions on root and choose Add note
    t.press('m').await;
    t.draw()?;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "add_note_prompt_open");

    // type new note name and submit
    for ch in "New Note".chars() {
        t.press(ch).await;
    }
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "note_added");

    Ok(())
}

#[tokio::test]
async fn add_directory_via_directory_actions() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // open directory actions on root and choose Add directory
    t.press('m').await;
    t.draw()?;
    t.press('j').await;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "add_dir_prompt_open");

    for ch in "Tmp".chars() {
        t.press(ch).await;
    }
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "dir_added");

    Ok(())
}

#[tokio::test]
async fn rename_note_prompt_and_apply() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // select first note and open note actions: default selection is Rename note
    t.press('j').await;
    t.press('m').await;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "rename_note_prompt_open");

    // clear default name and type new name
    for _ in 0..20 {
        t.key(KeyCode::Backspace).await;
    }
    for ch in "Renamed".chars() {
        t.press(ch).await;
    }
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "note_renamed");

    Ok(())
}

#[tokio::test]
async fn remove_note_confirm_cancel_then_accept() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // select note, open note actions, move to Remove note
    t.press('j').await;
    t.press('m').await;
    t.press('j').await;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "remove_note_confirm");

    // cancel once
    t.press('n').await;
    t.draw()?;
    snap!(t, "remove_note_cancelled");

    // remove for real
    t.press('m').await;
    t.press('j').await;
    t.key(KeyCode::Enter).await;
    t.press('y').await;
    t.draw()?;
    snap!(t, "note_removed");

    Ok(())
}

#[tokio::test]
async fn remove_directory_confirm_cancel_then_accept() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    // first add a directory so we can remove it
    t.press('m').await;
    t.draw()?;
    t.press('j').await; // Add directory
    t.key(KeyCode::Enter).await;
    for ch in "Tmp".chars() {
        t.press(ch).await;
    }
    t.key(KeyCode::Enter).await;
    t.draw()?;

    // select the new directory (first child under root)
    t.press('j').await;
    t.draw()?;

    // open directory actions and choose Remove directory
    t.press('m').await;
    t.press('j').await; // Add directory -> Rename directory
    t.press('j').await; // Rename directory -> Remove directory
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "remove_dir_confirm");

    // cancel
    t.press('n').await;
    t.draw()?;
    snap!(t, "remove_dir_cancelled");

    // confirm remove
    t.press('m').await;
    t.press('j').await;
    t.press('j').await;
    t.key(KeyCode::Enter).await;
    t.press('y').await;
    t.draw()?;
    snap!(t, "dir_removed");

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

 
