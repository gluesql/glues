use assert_cmd::cargo::cargo_bin;
use color_eyre::Result;
use expectrl::{Eof, Regex, session::Session};
use insta::assert_snapshot;
use std::process::Command;

#[test]
fn home_screen_snapshot() -> Result<()> {
    let bin = cargo_bin("glues");
    let cmd = Command::new(bin);
    let mut pty = Session::spawn(cmd)?;

    let found = pty.expect(Regex("Show keymap"))?;
    let screen = String::from_utf8_lossy(found.before()).to_string();
    assert_snapshot!("home_screen", screen);

    pty.send("q")?;
    pty.expect(Eof)?;
    Ok(())
}
