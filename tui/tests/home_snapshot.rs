use assert_cmd::cargo::cargo_bin;
use color_eyre::Result;
use expectrl::{Eof, Error, Regex, session::Session};
use insta::assert_debug_snapshot;
use nix::sys::signal::Signal;
use std::{
    process::Command,
    thread::sleep,
    time::{Duration, Instant},
};
use vt100::Parser;

fn drain_until_quiet(pty: &mut Session, parser: &mut Parser, quiet_ms: u64, timeout_ms: u64) {
    let mut buf = [0u8; 8192];
    let mut last_update = Instant::now();
    let start = Instant::now();
    loop {
        match pty.try_read(&mut buf) {
            Ok(n) if n > 0 => {
                parser.process(&buf[..n]);
                last_update = Instant::now();
            }
            _ => {
                // no bytes right now
                if last_update.elapsed() >= Duration::from_millis(quiet_ms) {
                    break;
                }
                if start.elapsed() >= Duration::from_millis(timeout_ms) {
                    break;
                }
                sleep(Duration::from_millis(10));
            }
        }
    }
}

#[test]
fn home_screen_snapshot() -> Result<()> {
    let bin = cargo_bin("glues");
    let cmd = Command::new(bin);
    let mut pty = Session::spawn(cmd)?;
    // ensure terminal has 120×40 size for predictable snapshots
    pty.get_process_mut().set_window_size(120, 40)?;
    // redraw after resizing so the initial frame uses the new dimensions
    pty.get_process_mut().signal(Signal::SIGWINCH)?;

    let mut parser = Parser::new(40, 120, 40);

    let first = loop {
        match pty.expect(Regex("Show keymap")) {
            Ok(m) => break m,
            Err(Error::Eof) => {
                sleep(Duration::from_millis(10));
            }
            Err(e) => return Err(e.into()),
        }
    };
    parser.process(first.as_bytes());

    // wait for the quit hint and capture the rest of the screen
    let menu = pty.expect(Regex("\\[q\\] Quit"))?;
    parser.process(menu.as_bytes());
    // drain updates until the UI is quiet so we snapshot the final frame
    drain_until_quiet(&mut pty, &mut parser, 150, 2000);
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
    // redraw after resizing so the notebook view fits the configured size
    pty.get_process_mut().signal(Signal::SIGWINCH)?;

    // capture the initial home screen
    let menu = pty.expect(Regex("\\[q\\] Quit"))?;
    let mut parser = Parser::new(40, 120, 40);
    parser.process(menu.as_bytes());

    // open the in-memory (Instant) storage and wait for notebook content
    pty.send("1")?;
    // expectrl may return `Eof` briefly while the TUI switches screens;
    // retry until the notebook banner appears.
    let notebook = loop {
        match pty.expect(Regex("Welcome to Glues :D")) {
            Ok(m) => break m,
            Err(Error::Eof) => {
                sleep(Duration::from_millis(10));
            }
            Err(e) => return Err(e.into()),
        }
    };
    parser.process(notebook.as_bytes());
    // drain updates until the UI is quiet so we snapshot the final frame
    drain_until_quiet(&mut pty, &mut parser, 150, 2000);
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
