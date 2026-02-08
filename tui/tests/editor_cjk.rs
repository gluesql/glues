#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;
use glues_tui::input::KeyCode;

/// Helper: clear the editor and type text, then go to col 0 in normal mode.
async fn setup(t: &mut Tester, text: &str) -> Result<()> {
    t.open_instant().await?;
    t.open_first_note().await?;

    // Delete default content, enter insert mode, type text
    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    t.type_str(text).await;
    t.key(KeyCode::Esc).await;

    // Go to beginning of line
    t.press('0').await;
    Ok(())
}

// "안녕 세계 테스트"
// 안=0, 녕=1, ' '=2, 세=3, 계=4, ' '=5, 테=6, 스=7, 트=8

#[tokio::test]
async fn word_forward_korean() -> Result<()> {
    let mut t = Tester::new().await?;
    setup(&mut t, "안녕 세계 테스트").await?;

    assert_eq!(t.cursor(), (0, 0)); // "안"

    t.press('w').await;
    assert_eq!(t.cursor(), (0, 3)); // "세"

    t.press('w').await;
    assert_eq!(t.cursor(), (0, 6)); // "테"

    // snapshot for visual reference
    t.draw()?;
    snap!(t, "korean_w_2");

    Ok(())
}

#[tokio::test]
async fn word_end_korean() -> Result<()> {
    let mut t = Tester::new().await?;
    setup(&mut t, "안녕 세계 테스트").await?;

    // e → end of "안녕"
    t.press('e').await;
    assert_eq!(t.cursor(), (0, 1)); // "녕"

    // e → end of "세계"
    t.press('e').await;
    assert_eq!(t.cursor(), (0, 4)); // "계"

    // e → end of "테스트"
    t.press('e').await;
    assert_eq!(t.cursor(), (0, 8)); // "트"

    Ok(())
}

#[tokio::test]
async fn word_back_korean() -> Result<()> {
    let mut t = Tester::new().await?;
    setup(&mut t, "안녕 세계 테스트").await?;

    // Go to end of last word
    t.press('$').await;
    let end_col = t.cursor().1;

    // b → start of "테스트"
    t.press('b').await;
    assert_eq!(t.cursor(), (0, 6)); // "테"

    // b → start of "세계"
    t.press('b').await;
    assert_eq!(t.cursor(), (0, 3)); // "세"

    // b → start of "안녕"
    t.press('b').await;
    assert_eq!(t.cursor(), (0, 0)); // "안"

    // verify the end position was reasonable
    assert!(end_col >= 8, "$ should go to end of line, got {end_col}");

    Ok(())
}

// "hello 세계! foo"
// h=0,e=1,l=2,l=3,o=4,' '=5,세=6,계=7,'!'=8,' '=9,f=10,o=11,o=12

#[tokio::test]
async fn word_motion_mixed() -> Result<()> {
    let mut t = Tester::new().await?;
    setup(&mut t, "hello 세계! foo").await?;

    assert_eq!(t.cursor(), (0, 0));

    // w → skip "hello" + space → "세"
    t.press('w').await;
    assert_eq!(t.cursor(), (0, 6));

    // w → skip "세계" → "!"
    t.press('w').await;
    assert_eq!(t.cursor(), (0, 8));

    // w → skip "!" + space → "foo"
    t.press('w').await;
    assert_eq!(t.cursor(), (0, 10));

    // b → "!"
    t.press('b').await;
    assert_eq!(t.cursor(), (0, 8));

    // b → "세"
    t.press('b').await;
    assert_eq!(t.cursor(), (0, 6));

    // b → "hello"
    t.press('b').await;
    assert_eq!(t.cursor(), (0, 0));

    // e → end of "hello"
    t.press('e').await;
    assert_eq!(t.cursor(), (0, 4));

    // e → end of "세계"
    t.press('e').await;
    assert_eq!(t.cursor(), (0, 7));

    Ok(())
}

#[tokio::test]
async fn delete_inner_word_korean() -> Result<()> {
    let mut t = Tester::new().await?;
    setup(&mut t, "안녕 세계 테스트").await?;

    // w → "세"
    t.press('w').await;
    assert_eq!(t.cursor(), (0, 3));

    // diw — delete "세계"
    t.press('d').await;
    t.press('i').await;
    t.press('w').await;

    assert_eq!(t.editor_text(), "안녕  테스트");

    Ok(())
}

#[tokio::test]
async fn change_inner_word_korean() -> Result<()> {
    let mut t = Tester::new().await?;
    setup(&mut t, "안녕 세계 테스트").await?;

    // w → "세"
    t.press('w').await;

    // ciw → delete "세계", enter insert mode
    t.press('c').await;
    t.press('i').await;
    t.press('w').await;
    t.type_str("지구").await;

    assert_eq!(t.editor_text(), "안녕 지구 테스트");

    Ok(())
}

#[tokio::test]
async fn word_forward_multiline_korean() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.press('d').await;
    t.press('d').await;
    t.press('i').await;
    t.type_str("가나다").await;
    t.key(KeyCode::Enter).await;
    t.type_str("라마바").await;
    t.key(KeyCode::Esc).await;
    t.press('g').await;
    t.press('g').await;
    t.press('0').await;

    assert_eq!(t.cursor(), (0, 0));

    // w on a word that extends to end of line → next line start
    t.press('w').await;
    assert_eq!(t.cursor(), (1, 0));

    Ok(())
}

#[tokio::test]
async fn delete_word_end_korean() -> Result<()> {
    let mut t = Tester::new().await?;
    setup(&mut t, "안녕 세계 테스트").await?;

    assert_eq!(t.cursor(), (0, 0));

    // de — delete to end of word (deletes "안녕", space remains)
    t.press('d').await;
    t.press('e').await;

    assert_eq!(t.editor_text(), " 세계 테스트");

    Ok(())
}
