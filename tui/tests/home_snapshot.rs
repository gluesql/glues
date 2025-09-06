use assert_cmd::cargo::cargo_bin;
use color_eyre::Result;
use expectrl::{session::Session, Eof, Regex};
use insta::assert_debug_snapshot;
use std::process::Command;
use vt100::Parser;

#[test]
fn home_screen_snapshot() -> Result<()> {
    let bin = cargo_bin("glues");
    let cmd = Command::new(bin);
    let mut pty = Session::spawn(cmd)?;
    // ensure terminal has 120x40 size for predictable snapshots
    pty.get_process_mut().set_window_size(120, 40)?;

    let first = pty.expect(Regex("Show keymap"))?;
    let mut output = first.before().to_vec();
    if let Some(m) = first.get(0) {
        output.extend_from_slice(m);
    }
    let menu = pty.expect(Regex("\\[q\\] Quit"))?;
    output.extend_from_slice(menu.before());
    if let Some(m) = menu.get(0) {
        output.extend_from_slice(m);
    }
    let mut parser = Parser::new(40, 120, 0);
    parser.process(&output);
    let screen = parser.screen().rows(0, 120).collect::<Vec<String>>();
    assert_debug_snapshot!("home_screen", screen);

    pty.send("q")?;
    pty.expect(Eof)?;
    Ok(())
}
