#[macro_use]
mod tester;
use tester::Tester;

use {color_eyre::Result, ratatui::crossterm::event::KeyCode, std::time::SystemTime};

#[tokio::test]
async fn notebook_browser_toggle() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.draw()?;
    snap!(t, "notebook_browser_visible");

    t.press('t').await;
    t.press('b').await;
    t.draw()?;
    snap!(t, "notebook_browser_hidden");

    t.press('t').await;
    t.press('b').await;

    Ok(())
}

#[tokio::test]
async fn notebook_editor_states() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.press('i').await;
    t.type_str("X").await;
    t.key(KeyCode::Esc).await;
    t.draw()?;
    snap!(t, "editor_dirty");

    {
        let context = t.app.context_mut();
        if let Some((_, item)) = context.notebook.editors.iter_mut().next() {
            item.dirty = false;
        }
        context.last_log = Some(("Synced".into(), SystemTime::now()));
    }
    t.draw()?;
    snap!(t, "editor_last_log");
    t.app.context_mut().last_log = None;

    t.press('i').await;
    t.draw()?;
    snap!(t, "editor_insert_mode");
    t.key(KeyCode::Esc).await;

    t.press('v').await;
    t.draw()?;
    snap!(t, "editor_visual_mode");
    t.key(KeyCode::Esc).await;

    t.press('t').await;
    t.press('n').await;
    t.draw()?;
    snap!(t, "editor_no_line_numbers");
    t.press('t').await;
    t.press('n').await;

    // Add a second note to create multiple tabs
    t.key(KeyCode::Tab).await;
    t.press('m').await;
    t.key(KeyCode::Enter).await;
    t.type_str("Second").await;
    t.key(KeyCode::Enter).await;
    t.press('j').await;
    t.press('l').await;

    // Highlight tabs while browsing the tree
    t.key(KeyCode::Tab).await;
    t.draw()?;
    snap!(t, "editor_tab_tree");
    t.press('l').await;

    // Create a nested directory and note to exercise breadcrumb styling
    t.key(KeyCode::Tab).await;
    t.press('k').await;
    t.press('k').await;
    t.press('m').await;
    t.press('j').await;
    t.key(KeyCode::Enter).await;
    t.type_str("Dir").await;
    t.key(KeyCode::Enter).await;
    t.press('j').await;
    t.press('l').await;
    t.press('m').await;
    t.key(KeyCode::Enter).await;
    t.type_str("Nested").await;
    t.key(KeyCode::Enter).await;
    t.press('j').await;
    t.press('l').await;
    t.key(KeyCode::Esc).await;
    {
        let context = t.app.context_mut();
        for (_, item) in context.notebook.editors.iter_mut() {
            item.dirty = false;
        }
    }
    t.draw()?;
    snap!(t, "editor_breadcrumb_nested");

    t.press('t').await;
    t.press('x').await;
    t.draw()?;
    snap!(t, "editor_inactive");

    Ok(())
}

#[tokio::test]
async fn vim_keymap_variants() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.key(KeyCode::Esc).await;
    t.ctrl('h').await;
    t.draw()?;
    snap!(t, "vim_keymap_normal_idle");
    t.press('x').await;

    t.press('2').await;
    t.ctrl('h').await;
    t.draw()?;
    snap!(t, "vim_keymap_normal_numbering");
    t.press('x').await;
    t.key(KeyCode::Esc).await;

    t.press('d').await;
    t.ctrl('h').await;
    t.draw()?;
    snap!(t, "vim_keymap_normal_delete");
    t.press('x').await;
    t.key(KeyCode::Esc).await;

    t.press('d').await;
    t.press('2').await;
    t.ctrl('h').await;
    t.draw()?;
    snap!(t, "vim_keymap_normal_delete2");
    t.press('x').await;
    t.key(KeyCode::Esc).await;

    t.press('c').await;
    t.ctrl('h').await;
    t.draw()?;
    snap!(t, "vim_keymap_normal_change");
    t.press('x').await;
    t.key(KeyCode::Esc).await;

    t.press('c').await;
    t.press('2').await;
    t.ctrl('h').await;
    t.draw()?;
    snap!(t, "vim_keymap_normal_change2");
    t.press('x').await;
    t.key(KeyCode::Esc).await;

    t.press('v').await;
    t.ctrl('h').await;
    t.draw()?;
    snap!(t, "vim_keymap_visual_idle");
    t.press('x').await;
    t.key(KeyCode::Esc).await;

    t.press('v').await;
    t.press('2').await;
    t.ctrl('h').await;
    t.draw()?;
    snap!(t, "vim_keymap_visual_numbering");
    t.press('x').await;
    t.key(KeyCode::Esc).await;

    Ok(())
}
