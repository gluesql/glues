#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;
use glues_tui::input::KeyCode;

/// Helper: clear the editor and type text, then go to col 0 in normal mode.
async fn setup(t: &mut Tester, text: &str) -> Result<()> {
    t.open_instant().await?;
    t.open_first_note().await?;

    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    t.type_str(text).await;
    t.key(KeyCode::Esc).await;

    t.press('0').await;
    Ok(())
}

/// Helper: clear the editor and type multiple lines, then go to (0, 0) in normal mode.
async fn setup_lines(t: &mut Tester, lines: &[&str]) -> Result<()> {
    t.open_instant().await?;
    t.open_first_note().await?;

    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            t.key(KeyCode::Enter).await;
        }
        t.type_str(line).await;
    }
    t.key(KeyCode::Esc).await;

    t.press('g').await;
    t.press('g').await;
    t.press('0').await;
    Ok(())
}

#[tokio::test]
async fn word_forward_skips_next_line_whitespace() -> Result<()> {
    let mut t = Tester::new().await?;
    setup_lines(&mut t, &["hello", "  world"]).await?;

    assert_eq!(t.cursor(), (0, 0));

    // w → "hello" extends to end of line → next line, skip leading spaces
    t.press('w').await;
    assert_eq!(t.cursor(), (1, 2)); // "w" in "  world"

    Ok(())
}

#[tokio::test]
async fn word_forward_trailing_whitespace_crosses_line() -> Result<()> {
    let mut t = Tester::new().await?;
    setup_lines(&mut t, &["hi   ", "world"]).await?;

    assert_eq!(t.cursor(), (0, 0));

    // w → skip "hi", trailing spaces extend to end of line → cross to next line
    t.press('w').await;
    assert_eq!(t.cursor(), (1, 0));

    Ok(())
}

#[tokio::test]
async fn word_back_cross_line() -> Result<()> {
    let mut t = Tester::new().await?;
    setup_lines(&mut t, &["hello world", "foo"]).await?;

    // Move to start of "foo" on line 1
    t.press('j').await;
    t.press('0').await;
    assert_eq!(t.cursor(), (1, 0));

    // b → start of "world" on prev line
    t.press('b').await;
    assert_eq!(t.cursor(), (0, 6));

    // b → start of "hello"
    t.press('b').await;
    assert_eq!(t.cursor(), (0, 0));

    Ok(())
}

#[tokio::test]
async fn delete_word_back_exclusive() -> Result<()> {
    let mut t = Tester::new().await?;
    setup(&mut t, "hello world").await?;

    // Move to "world"
    t.press('w').await;
    assert_eq!(t.cursor(), (0, 6));

    // db — delete "hello " (exclusive: "w" not included)
    t.press('d').await;
    t.press('b').await;

    assert_eq!(t.editor_text(), "world");

    Ok(())
}

// -- w from whitespace / punctuation --

#[tokio::test]
async fn word_forward_from_whitespace() -> Result<()> {
    let mut t = Tester::new().await?;
    // "hello   world"
    // h=0 e=1 l=2 l=3 o=4 ' '=5 ' '=6 ' '=7 w=8
    setup(&mut t, "hello   world").await?;

    // move to first space
    t.press('w').await;
    assert_eq!(t.cursor(), (0, 8)); // w skips "hello" + whitespace → "world"

    Ok(())
}

#[tokio::test]
async fn word_forward_from_punctuation() -> Result<()> {
    let mut t = Tester::new().await?;
    // "foo!!! bar"
    // f=0 o=1 o=2 !=3 !=4 !=5 ' '=6 b=7
    setup(&mut t, "foo!!! bar").await?;

    // w → skip "foo" → "!!!"
    t.press('w').await;
    assert_eq!(t.cursor(), (0, 3));

    // w → skip "!!!" + space → "bar"
    t.press('w').await;
    assert_eq!(t.cursor(), (0, 7));

    Ok(())
}

// -- e cross-line --

#[tokio::test]
async fn word_end_cross_line() -> Result<()> {
    let mut t = Tester::new().await?;
    setup_lines(&mut t, &["hello", "world"]).await?;

    // e → end of "hello" = col 4
    t.press('e').await;
    assert_eq!(t.cursor(), (0, 4));

    // e → end of "world" on next line = (1, 4)
    t.press('e').await;
    assert_eq!(t.cursor(), (1, 4));

    Ok(())
}

#[tokio::test]
async fn word_end_skips_whitespace() -> Result<()> {
    let mut t = Tester::new().await?;
    // "hi   world"
    // h=0 i=1 ' '=2 ' '=3 ' '=4 w=5
    setup(&mut t, "hi   world").await?;

    // e → end of "hi" = col 1
    t.press('e').await;
    assert_eq!(t.cursor(), (0, 1));

    // e → skip whitespace → end of "world" = col 9
    t.press('e').await;
    assert_eq!(t.cursor(), (0, 9));

    Ok(())
}

// -- b from leading whitespace (cross-line branch) --

#[tokio::test]
async fn word_back_from_leading_whitespace() -> Result<()> {
    let mut t = Tester::new().await?;
    setup_lines(&mut t, &["hello", "   world"]).await?;

    // move to line 1, col 0 (leading space)
    t.press('j').await;
    t.press('0').await;
    assert_eq!(t.cursor(), (1, 0));

    // b from whitespace → should cross to prev line, start of "hello"
    t.press('b').await;
    assert_eq!(t.cursor(), (0, 0));

    Ok(())
}

// -- diw on whitespace / punctuation --

#[tokio::test]
async fn delete_inner_word_on_whitespace() -> Result<()> {
    let mut t = Tester::new().await?;
    // "hello   world"
    setup(&mut t, "hello   world").await?;

    // move cursor to whitespace region (use l to go to col 5)
    t.press('w').await; // lands on "world" (col 8) because w skips ws
    // go back to col 5 with b then l
    t.press('b').await; // back to "hello" (col 0)
    for _ in 0..5 {
        t.press('l').await;
    }
    assert_eq!(t.cursor(), (0, 5)); // on first space

    // diw on whitespace — should delete the whitespace block "   "
    t.press('d').await;
    t.press('i').await;
    t.press('w').await;

    assert_eq!(t.editor_text(), "helloworld");

    Ok(())
}

#[tokio::test]
async fn delete_inner_word_on_punctuation() -> Result<()> {
    let mut t = Tester::new().await?;
    // "hello...world"
    // h=0 e=1 l=2 l=3 o=4 .=5 .=6 .=7 w=8
    setup(&mut t, "hello...world").await?;

    // move to first dot (col 5)
    for _ in 0..5 {
        t.press('l').await;
    }
    assert_eq!(t.cursor(), (0, 5));

    // diw on punctuation — should delete "..."
    t.press('d').await;
    t.press('i').await;
    t.press('w').await;

    assert_eq!(t.editor_text(), "helloworld");

    Ok(())
}
