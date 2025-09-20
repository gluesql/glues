#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;
use glues_tui::input::KeyCode;

#[tokio::test]
async fn quits_on_esc_then_q() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    t.open_first_note().await?;

    // Esc then 'q' should quit
    t.key(KeyCode::Esc).await;
    let quit = t.press('q').await;
    assert!(quit);

    Ok(())
}

#[tokio::test]
async fn quit_menu_cancel_from_normal() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.key(KeyCode::Esc).await;
    t.draw()?;
    snap!(t, "quit_menu_open");

    t.key(KeyCode::Esc).await;
    t.draw()?;
    snap!(t, "quit_menu_closed");
    Ok(())
}

#[tokio::test]
async fn quit_menu_returns_to_entry() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;

    t.key(KeyCode::Esc).await;
    let quit = t.press('m').await;
    assert!(!quit);

    t.draw()?;
    snap!(t, "entry_after_quit_menu");
    Ok(())
}

#[tokio::test]
async fn undo_then_redo_after_insert() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // insert text
    t.press('i').await;
    t.type_str("Hello").await;
    t.key(KeyCode::Esc).await;
    t.draw()?;
    snap!(t, "text_after_insert");

    // undo (u)
    t.press('u').await;
    t.draw()?;
    snap!(t, "after_undo");

    // redo (Ctrl+r)
    t.ctrl('r').await;
    t.draw()?;
    snap!(t, "after_redo");

    Ok(())
}

#[tokio::test]
async fn yank_line_then_paste() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // yy then p
    t.press('y').await;
    t.press('y').await;
    t.press('p').await;
    t.draw()?;
    snap!(t, "after_yank_paste");

    Ok(())
}

#[tokio::test]
async fn delete_line_with_dd() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.press('d').await;
    t.press('d').await;
    t.draw()?;
    snap!(t, "after_delete_line");

    Ok(())
}
