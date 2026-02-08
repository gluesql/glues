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
async fn delete_inside_word() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // replace default content with multi-word text
    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    t.type_str("hello world foo bar").await;
    t.key(KeyCode::Esc).await;

    // move to "world" (w w)
    t.press('0').await;
    t.press('w').await;
    t.draw()?;
    snap!(t, "diw_before");

    // diw — delete "world"
    t.press('d').await;
    t.press('i').await;
    t.press('w').await;
    t.draw()?;
    snap!(t, "diw_after");

    // undo and move back to "world" for 2diw test
    t.press('u').await;
    t.press('0').await;
    t.press('w').await;

    // 2diw — delete two words
    t.press('2').await;
    t.press('d').await;
    t.press('i').await;
    t.press('w').await;
    t.draw()?;
    snap!(t, "2diw_after");

    Ok(())
}

#[tokio::test]
async fn change_inside_word() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // replace default content with multi-word text
    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    t.type_str("hello world foo bar").await;
    t.key(KeyCode::Esc).await;

    // move to "world"
    t.press('0').await;
    t.press('w').await;

    // ciw — change "world" and type replacement
    t.press('c').await;
    t.press('i').await;
    t.press('w').await;
    t.type_str("planet").await;
    t.draw()?;
    snap!(t, "ciw_after");

    // undo and move back to "world" for 2ciw test
    t.key(KeyCode::Esc).await;
    t.press('u').await;
    t.press('0').await;
    t.press('w').await;

    // 2ciw — change two words and type replacement
    t.press('2').await;
    t.press('c').await;
    t.press('i').await;
    t.press('w').await;
    t.type_str("earth").await;
    t.draw()?;
    snap!(t, "2ciw_after");

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
