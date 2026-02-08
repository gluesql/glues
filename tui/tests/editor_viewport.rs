#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;
use glues_tui::input::KeyCode;

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

#[tokio::test]
async fn scroll_zt_then_jk_stable_viewport() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    populate_long_note(&mut t).await?;

    // Go to end of document
    t.press('G').await;

    // zt: scroll cursor line to top
    t.press('z').await;
    t.press('t').await;
    t.draw()?;
    snap!(t, "zt_at_bottom");

    // j at last line (cursor stays, viewport must not jump)
    t.press('j').await;
    t.draw()?;
    snap!(t, "zt_at_bottom_after_j");

    // k moves cursor up one line, viewport stays stable
    t.press('k').await;
    t.draw()?;
    snap!(t, "zt_at_bottom_after_k");

    // k 4 more times
    for _ in 0..4 {
        t.press('k').await;
    }
    t.draw()?;
    snap!(t, "zt_at_bottom_after_5k");

    Ok(())
}

#[tokio::test]
async fn scroll_zt_then_k_releases_anchor() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    populate_long_note(&mut t).await?;

    // Go to end and zt
    t.press('G').await;
    t.press('z').await;
    t.press('t').await;
    t.draw()?;
    snap!(t, "zt_release_start");

    // k 20 times — cursor moves up enough to release anchor
    for _ in 0..20 {
        t.press('k').await;
    }
    t.draw()?;
    snap!(t, "zt_release_after_20k");

    // j 5 times — anchor released, normal viewport behavior
    for _ in 0..5 {
        t.press('j').await;
    }
    t.draw()?;
    snap!(t, "zt_release_after_j");

    Ok(())
}

#[tokio::test]
async fn scroll_zz_midfile_jk_no_shift() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    populate_long_note(&mut t).await?;

    // Go to top, then move to row 20
    t.press('g').await;
    t.press('g').await;
    for _ in 0..20 {
        t.press('j').await;
    }

    // zz (z.)
    t.press('z').await;
    t.press('.').await;
    t.draw()?;
    snap!(t, "zz_mid_start");

    // j 3 times
    for _ in 0..3 {
        t.press('j').await;
    }
    t.draw()?;
    snap!(t, "zz_mid_after_3j");

    // k 3 times — back to original position
    for _ in 0..3 {
        t.press('k').await;
    }
    t.draw()?;
    snap!(t, "zz_mid_after_3k");

    Ok(())
}
