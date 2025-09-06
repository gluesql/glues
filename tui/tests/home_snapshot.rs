use assert_cmd::cargo::cargo_bin;
use color_eyre::Result;
use expectrl::{Eof, Regex, session::Session};
use insta::assert_debug_snapshot;
use std::{io::Read, process::Command};
use vt100::Parser;

#[test]
fn home_screen_snapshot() -> Result<()> {
    let bin = cargo_bin("glues");
    let cmd = Command::new(bin);
    let mut pty = Session::spawn(cmd)?;
    // ensure terminal has 120x40 size for predictable snapshots
    pty.get_process_mut().set_window_size(120, 40)?;

    let first = pty.expect(Regex("Show keymap"))?;
    let mut output = first.as_bytes().to_vec();

    // wait for the quit hint and capture the rest of the screen
    let menu = pty.expect(Regex("\\[q\\] Quit"))?;
    output.extend_from_slice(menu.as_bytes());
    let mut tail = [0u8; 1024];
    let n = pty.read(&mut tail)?;
    output.extend_from_slice(&tail[..n]);

    let mut parser = Parser::new(40, 120, 0);
    parser.process(&output);
    let screen = parser.screen().rows(0, 40).collect::<Vec<String>>();
    assert_debug_snapshot!("home_screen", screen);

    pty.send("q")?;
    pty.expect(Eof)?;
    Ok(())
}
