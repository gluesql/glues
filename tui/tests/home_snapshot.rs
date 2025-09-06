use assert_cmd::cargo::cargo_bin;
use color_eyre::Result;
use expectrl::{Eof, Regex, session::Session};
use insta::assert_debug_snapshot;
use std::process::Command;
use vt100::Parser;

#[test]
fn home_screen_snapshot() -> Result<()> {
    let bin = cargo_bin("glues");
    let cmd = Command::new(bin);
    let mut pty = Session::spawn(cmd)?;
    // ensure terminal has 80x24 size for predictable snapshots
    pty.get_process_mut().set_window_size(80, 24)?;

    let found = pty.expect(Regex("Show keymap"))?;
    let mut output = found.before().to_vec();
    if let Some(m) = found.get(0) {
        output.extend_from_slice(m);
    }
    let mut parser = Parser::new(24, 80, 0);
    parser.process(&output);
    let screen = parser.screen().rows(0, 80).collect::<Vec<String>>();
    assert_debug_snapshot!("home_screen", screen);

    pty.send("q")?;
    pty.expect(Eof)?;
    Ok(())
}
