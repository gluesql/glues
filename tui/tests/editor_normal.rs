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

#[tokio::test]
async fn delete_word_with_dw() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // Insert some text with multiple words
    t.press('i').await;
    t.type_str("Hello world test").await;
    t.key(KeyCode::Esc).await;

    // Move to beginning of line
    t.press('0').await;

    // Delete word with 'dw'
    t.press('d').await;
    t.press('w').await;
    t.draw()?;
    snap!(t, "after_delete_word_dw");

    Ok(())
}

#[tokio::test]
async fn delete_word_with_de() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // Insert some text with multiple words
    t.press('i').await;
    t.type_str("Hello world test").await;
    t.key(KeyCode::Esc).await;

    // Move to beginning of line
    t.press('0').await;

    // Delete word with 'de'
    t.press('d').await;
    t.press('e').await;
    t.draw()?;
    snap!(t, "after_delete_word_de");

    Ok(())
}

#[tokio::test]
async fn gateway_moves_cursor_to_top() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    populate_long_note(&mut t).await?;
    t.draw()?;

    // Move to bottom so the gateway jump visibly changes the view.
    t.press('G').await;

    // First 'g' enters gateway mode.
    t.press('g').await;
    t.draw()?;
    snap!(t, "gateway_prompt");

    // Second 'g' jumps to the top line.
    t.press('g').await;
    t.draw()?;
    snap!(t, "gateway_move_top");

    Ok(())
}

#[tokio::test]
async fn scroll_commands_update_view() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    populate_long_note(&mut t).await?;
    t.draw()?;

    // Start near the middle of the document.
    t.press('g').await;
    t.press('g').await;
    for _ in 0..20 {
        t.press('j').await;
    }

    t.press('z').await;
    t.draw()?;
    snap!(t, "scroll_prompt");

    t.press('t').await;
    t.draw()?;
    snap!(t, "scroll_jump_top");

    // Jump to bottom and use 'zb'.
    t.press('G').await;
    t.press('z').await;
    t.press('b').await;
    t.draw()?;
    snap!(t, "scroll_jump_bottom");

    // Move back toward the middle and center with 'z.'.
    for _ in 0..15 {
        t.press('k').await;
    }
    t.press('z').await;
    t.press('.').await;
    t.draw()?;
    snap!(t, "scroll_jump_center");

    Ok(())
}

async fn populate_long_note(tester: &mut Tester) -> Result<()> {
    let lines: Vec<String> = (1..=60)
        .map(|i| format!("Line {i:02} — the quick brown fox jumps over the lazy dog."))
        .collect();

    tester.press('g').await;
    tester.press('g').await;

    tester.press('d').await;
    tester.press('d').await;

    tester.press('i').await;
    for (idx, line) in lines.iter().enumerate() {
        tester.type_str(line).await;
        if idx + 1 < lines.len() {
            tester.key(KeyCode::Enter).await;
        }
    }
    tester.key(KeyCode::Esc).await;

    tester.key(KeyCode::Tab).await;
    tester.draw()?;
    tester.key(KeyCode::Tab).await;

    Ok(())
}
