#[macro_use]
mod tester;
use tester::Tester;

use color_eyre::Result;
use ratatui::crossterm::event::KeyCode;

#[tokio::test]
async fn enter_visual_mode_and_escape() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.press('v').await;
    t.draw()?;
    snap!(t, "visual_mode");

    t.key(KeyCode::Esc).await;
    t.draw()?;
    snap!(t, "visual_back_to_normal");

    Ok(())
}

#[tokio::test]
async fn visual_numbering_move() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.press('v').await;
    t.press('2').await;
    t.draw()?;
    snap!(t, "visual_numbering");

    t.press('j').await;
    t.draw()?;
    snap!(t, "visual_move_2_down");

    Ok(())
}

#[tokio::test]
async fn visual_gateway_mode_top() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.press('v').await;
    t.press('g').await;
    t.draw()?;
    snap!(t, "visual_gateway");

    t.press('g').await;
    t.draw()?;
    snap!(t, "visual_move_top");

    Ok(())
}

#[tokio::test]
async fn editor_keymap_overlay_in_visual_mode() -> Result<()> {
    let mut t = Tester::new().await?;
    t.open_instant().await?;
    t.open_first_note().await?;

    t.press('v').await;
    t.ctrl('h').await;
    t.draw()?;
    snap!(t, "visual_keymap_open");

    t.press('x').await;
    t.draw()?;
    snap!(t, "visual_keymap_closed");

    Ok(())
}
