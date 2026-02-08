#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;
use glues_tui::input::KeyCode;

#[tokio::test]
async fn long_line_wraps() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // delete default content and type a long line (120 chars) that exceeds editor width (~74 chars)
    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    t.type_str(&"abcdefgh".repeat(15)).await;
    t.draw()?;
    snap!(t, "long_line_wraps");

    Ok(())
}

#[tokio::test]
async fn multiple_long_lines_wrap() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // delete default content
    t.press('d').await;
    t.press('d').await;
    t.press('i').await;

    // type three long lines separated by Enter
    for i in 0..3 {
        let ch = (b'A' + i) as char;
        t.type_str(&format!("{ch}").repeat(100)).await;
        if i < 2 {
            t.key(KeyCode::Enter).await;
        }
    }

    t.draw()?;
    snap!(t, "multiple_long_lines_wrap");

    Ok(())
}

#[tokio::test]
async fn cursor_on_wrapped_line() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // delete default content and type a long line
    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    t.type_str(&"abcdefgh".repeat(15)).await;

    // return to normal mode so cursor is visible on the wrapped line
    t.key(KeyCode::Esc).await;
    t.draw()?;
    snap!(t, "cursor_on_wrapped_line");

    Ok(())
}

#[tokio::test]
async fn delete_word_on_wrapped_line() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // replace default content with a long line
    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    t.type_str("abcdefgh ".repeat(12).trim_end()).await;
    t.key(KeyCode::Esc).await;

    // go to beginning and delete a word with dw
    t.press('0').await;
    t.draw()?;
    snap!(t, "dw_before");

    t.press('d').await;
    t.press('w').await;
    t.draw()?;
    snap!(t, "dw_after");

    Ok(())
}

#[tokio::test]
async fn dd_on_wrapped_lines() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // replace default content with two long lines
    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    t.type_str(&"A".repeat(100)).await;
    t.key(KeyCode::Enter).await;
    t.type_str(&"B".repeat(100)).await;
    t.key(KeyCode::Esc).await;

    // go to first line
    t.press('g').await;
    t.press('g').await;
    t.draw()?;
    snap!(t, "dd_before");

    // dd deletes the entire first wrapped line
    t.press('d').await;
    t.press('d').await;
    t.draw()?;
    snap!(t, "dd_after");

    // undo restores it
    t.press('u').await;
    t.draw()?;
    snap!(t, "dd_undo");

    Ok(())
}

#[tokio::test]
async fn insert_middle_of_wrapped_line() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    // replace default content with a long line
    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    t.type_str(&"X".repeat(100)).await;
    t.key(KeyCode::Esc).await;

    // move cursor to middle of line and insert text
    t.press('0').await;
    for _ in 0..50 {
        t.press('l').await;
    }
    t.press('i').await;
    t.type_str("INSERTED").await;
    t.draw()?;
    snap!(t, "insert_middle_wrapped");

    Ok(())
}
