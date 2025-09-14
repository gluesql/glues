use color_eyre::Result;
use insta::assert_debug_snapshot;
mod common;
use common::{DEFAULT_HEIGHT, DEFAULT_QUIET_MS, DEFAULT_TIMEOUT_MS, DEFAULT_WIDTH, TuiHarness};

#[test]
fn home_screen_snapshot() -> Result<()> {
    let mut h = TuiHarness::spawn(DEFAULT_WIDTH, DEFAULT_HEIGHT)?;
    h.wait_for("Show keymap")?;
    h.wait_for("\\[q\\] Quit")?;
    h.drain_until_quiet(
        std::time::Duration::from_millis(DEFAULT_QUIET_MS),
        std::time::Duration::from_millis(DEFAULT_TIMEOUT_MS),
    );
    let snapshot = h.snapshot();
    assert_debug_snapshot!("home_screen", snapshot);

    h.send("q")?;
    h.expect_eof()?;
    Ok(())
}

#[test]
fn instant_screen_snapshot() -> Result<()> {
    let mut h = TuiHarness::spawn(DEFAULT_WIDTH, DEFAULT_HEIGHT)?;
    h.wait_for("\\[q\\] Quit")?;
    h.send("1")?;
    h.wait_for("Welcome to Glues :D")?;
    h.drain_until_quiet(
        std::time::Duration::from_millis(DEFAULT_QUIET_MS),
        std::time::Duration::from_millis(DEFAULT_TIMEOUT_MS),
    );
    let snapshot = h.snapshot();
    assert_debug_snapshot!("instant_screen", snapshot);

    h.send_ctrl_c()?; // Ctrl+C
    h.expect_eof()?;
    Ok(())
}
