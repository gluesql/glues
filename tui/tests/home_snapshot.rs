use assert_cmd::cargo::cargo_bin;
use color_eyre::Result;
use expectrl::{Eof, Regex, session::Session};
use insta::assert_debug_snapshot;
use std::{process::Command, thread::sleep, time::Duration};
use vt100::Parser;

#[test]
fn home_screen_snapshot() -> Result<()> {
    let bin = cargo_bin("glues");
    let cmd = Command::new(bin);
    let mut pty = Session::spawn(cmd)?;
    // ensure terminal has 120×40 size for predictable snapshots
    pty.get_process_mut().set_window_size(120, 40)?;

    let first = pty.expect(Regex("Show keymap"))?;
    let mut output = first.as_bytes().to_vec();

    // wait for the quit hint and capture the rest of the screen
    let menu = pty.expect(Regex("\\[q\\] Quit"))?;
    output.extend_from_slice(menu.as_bytes());
    // read any remaining bytes in the PTY buffer without blocking
    let mut buf = [0u8; 8192];
    while let Ok(n) = pty.try_read(&mut buf) {
        if n == 0 {
            break;
        }
        output.extend_from_slice(&buf[..n]);
    }

    let mut parser = Parser::new(40, 120, 40);
    parser.process(&output);
    let screen = parser.screen();
    let (height, width) = screen.size();
    let snapshot = screen
        .rows(0, width)
        .take(height as usize)
        .collect::<Vec<String>>();
    assert_debug_snapshot!("home_screen", snapshot);

    pty.send("q")?;
    pty.expect(Eof)?;
    Ok(())
}

#[test]
fn instant_screen_snapshot() -> Result<()> {
    let bin = cargo_bin("glues");
    let cmd = Command::new(bin);
    let mut pty = Session::spawn(cmd)?;
    // ensure terminal has 120×40 size for predictable snapshots
    pty.get_process_mut().set_window_size(120, 40)?;

    // capture the initial home screen
    let menu = pty.expect(Regex("\\[q\\] Quit"))?;
    let mut output = menu.as_bytes().to_vec();

    // open the in-memory (Instant) storage and wait for notebook content
    pty.send("1")?;
    let notebook = pty.expect(Regex("Welcome to Glues :D"))?;
    output.extend_from_slice(notebook.as_bytes());
    // allow remaining bytes to arrive
    sleep(Duration::from_millis(50));
    let mut buf = [0u8; 8192];
    while let Ok(n) = pty.try_read(&mut buf) {
        if n == 0 {
            break;
        }
        output.extend_from_slice(&buf[..n]);
    }

    let mut parser = Parser::new(40, 120, 40);
    parser.process(&output);
    let screen = parser.screen();
    let (height, width) = screen.size();
    let snapshot = screen
        .rows(0, width)
        .take(height as usize)
        .collect::<Vec<String>>();
    assert_debug_snapshot!("instant_screen", snapshot);

    pty.send("\u{3}")?; // Ctrl+C to quit
    pty.expect(Eof)?;
    Ok(())
}
