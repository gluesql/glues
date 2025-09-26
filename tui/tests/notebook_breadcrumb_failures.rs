#[macro_use]
mod tester;
use tester::Tester;

use {
    color_eyre::Result,
    glues_core::{Event, NotebookEvent},
    glues_tui::input::KeyCode,
};

#[tokio::test]
async fn closing_tab_then_reopening_still_panics() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    t.press('j').await;
    t.press('l').await;
    t.draw()?;
    snap!(t, "note_open");

    t.key(KeyCode::Tab).await;
    t.press('k').await;
    t.press('k').await;

    t.press('m').await;
    t.draw()?;
    t.press('j').await;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    for ch in "Tmp".chars() {
        t.press(ch).await;
    }
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "dir_added");

    t.press('m').await;
    t.draw()?;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    for ch in "Workspace".chars() {
        t.press(ch).await;
    }
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "workspace_note_open");

    t.press('t').await;
    t.press('x').await;
    t.draw()?;

    t.key(KeyCode::Tab).await;
    t.draw()?;

    t.press('k').await;
    t.draw()?;

    t.press('l').await;
    t.draw()?;
    snap!(t, "workspace_note_reopen");

    Ok(())
}

#[tokio::test]
async fn moving_note_between_directories_still_panics() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    t.press('j').await;
    t.press('l').await;
    t.draw()?;
    snap!(t, "note_open");

    t.key(KeyCode::Tab).await;
    t.press('k').await;
    t.press('k').await;

    t.press('m').await;
    t.draw()?;
    t.press('j').await;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    snap!(t, "moving_note_open");
    for ch in "Src".chars() {
        t.press(ch).await;
    }
    t.key(KeyCode::Enter).await;
    t.draw()?;

    t.press('m').await;
    t.draw()?;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    for ch in "Moving".chars() {
        t.press(ch).await;
    }
    t.key(KeyCode::Enter).await;
    t.draw()?;

    t.key(KeyCode::Tab).await;
    t.press('k').await;
    t.press('k').await;

    t.press('m').await;
    t.draw()?;
    t.press('j').await;
    t.key(KeyCode::Enter).await;
    t.draw()?;
    for ch in "Dst".chars() {
        t.press(ch).await;
    }
    t.key(KeyCode::Enter).await;
    t.draw()?;

    t.press('j').await;
    t.press('l').await;
    t.draw()?;
    t.key(KeyCode::Tab).await;

    t.key(KeyCode::Char(' ')).await;
    t.draw()?;
    t.press('j').await;
    t.draw()?;
    t.key(KeyCode::Enter).await;
    t.draw()?;

    t.press('k').await;
    t.draw()?;

    t.press('l').await;
    t.draw()?;
    snap!(t, "moving_note_after_move");

    Ok(())
}

#[tokio::test]
async fn moving_note_into_external_directory_updates_tree() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    t.press('j').await;
    t.press('l').await;
    t.draw()?;

    let directory_id = {
        let glues = t.app.glues_mut();
        let db = glues.db.as_mut().expect("backend must be initialized");
        let root_id = db.root_id();
        db.add_directory(root_id, "External Dir".to_owned())
            .await
            .expect("failed to add directory")
            .id
    };

    t.key(KeyCode::Tab).await;
    t.key(KeyCode::Char(' ')).await;
    t.draw()?;

    {
        let glues = t.app.glues_mut();
        glues
            .dispatch(Event::Notebook(NotebookEvent::MoveNote(directory_id)))
            .await
            .expect("move note should succeed");
    }

    t.draw()?;
    snap!(t, "external_directory_move");

    Ok(())
}
