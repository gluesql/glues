use assert_cmd::cargo::cargo_bin;
use color_eyre::Result;
use expectrl::{Eof, Error, Regex, session::Session};
use nix::sys::signal::Signal;
use std::{
    process::Command,
    thread::sleep,
    time::{Duration, Instant},
};
use vt100::Parser;

pub struct TuiHarness {
    pub pty: Session,
    pub parser: Parser,
    width: u16,
    height: u16,
}

impl TuiHarness {
    pub fn spawn(width: u16, height: u16) -> Result<Self> {
        let bin = cargo_bin("glues");
        let cmd = Command::new(bin);
        let mut pty = Session::spawn(cmd)?;
        pty.get_process_mut().set_window_size(width, height)?;
        pty.get_process_mut().signal(Signal::SIGWINCH)?;
        let parser = Parser::new(height, width, height as usize);
        Ok(Self {
            pty,
            parser,
            width,
            height,
        })
    }

    pub fn wait_for(&mut self, pattern: &str) -> Result<()> {
        loop {
            match self.pty.expect(Regex(pattern)) {
                Ok(m) => {
                    self.parser.process(m.as_bytes());
                    return Ok(());
                }
                Err(Error::Eof) => sleep(Duration::from_millis(10)),
                Err(e) => return Err(e.into()),
            }
        }
    }

    pub fn send(&mut self, s: &str) -> Result<()> {
        self.pty.send(s)?;
        Ok(())
    }

    pub fn send_ctrl_c(&mut self) -> Result<()> {
        self.pty.send("\u{3}")?;
        Ok(())
    }

    pub fn drain_until_quiet(&mut self, quiet: Duration, timeout: Duration) {
        let mut buf = [0u8; 8192];
        let mut last_update = Instant::now();
        let start = Instant::now();
        loop {
            match self.pty.try_read(&mut buf) {
                Ok(n) if n > 0 => {
                    self.parser.process(&buf[..n]);
                    last_update = Instant::now();
                }
                _ => {
                    if last_update.elapsed() >= quiet || start.elapsed() >= timeout {
                        break;
                    }
                    sleep(Duration::from_millis(10));
                }
            }
        }
    }

    pub fn snapshot(&self) -> Vec<String> {
        let screen = self.parser.screen();
        let (height, width) = screen.size();
        screen
            .rows(0, width)
            .take(height as usize)
            .collect::<Vec<String>>()
    }

    pub fn expect_eof(mut self) -> Result<()> {
        self.pty.expect(Eof)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

pub const DEFAULT_WIDTH: u16 = 120;
pub const DEFAULT_HEIGHT: u16 = 40;
pub const DEFAULT_QUIET_MS: u64 = 150;
pub const DEFAULT_TIMEOUT_MS: u64 = 2000;
