#![allow(async_fn_in_trait)]

use color_eyre::Result;
use ratatui::{
    Terminal,
    backend::TestBackend,
    crossterm::event::{
        Event as Input, KeyCode, KeyEvent as CKeyEvent, KeyEventKind, KeyModifiers,
    },
};

use glues::App;

async fn process_input(app: &mut App, input: Input) -> bool {
    if !matches!(
        input,
        Input::Key(CKeyEvent {
            kind: KeyEventKind::Press,
            ..
        })
    ) {
        return false;
    }

    match input {
        Input::Key(CKeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => true,
        _ => {
            let action = app.context_mut().consume(&input).await;
            app.handle_action(action, input).await
        }
    }
}

pub trait AppTestExt {
    fn draw_frame(&mut self, term: &mut Terminal<TestBackend>) -> color_eyre::Result<()>;
    async fn press(&mut self, c: char) -> bool;
    async fn ctrl(&mut self, c: char) -> bool;
    async fn key(&mut self, code: KeyCode) -> bool;
    async fn open_instant(&mut self, term: &mut Terminal<TestBackend>) -> Result<()>;
}

impl AppTestExt for App {
    fn draw_frame(&mut self, term: &mut Terminal<TestBackend>) -> color_eyre::Result<()> {
        term.draw(|f| self.draw(f))?;
        Ok(())
    }

    async fn press(&mut self, c: char) -> bool {
        process_input(
            self,
            Input::Key(CKeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE)),
        )
        .await
    }

    async fn ctrl(&mut self, c: char) -> bool {
        process_input(
            self,
            Input::Key(CKeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)),
        )
        .await
    }

    async fn key(&mut self, code: KeyCode) -> bool {
        process_input(self, Input::Key(CKeyEvent::new(code, KeyModifiers::NONE))).await
    }

    async fn open_instant(&mut self, term: &mut Terminal<TestBackend>) -> Result<()> {
        term.draw(|f| self.draw(f))?;
        process_input(
            self,
            Input::Key(CKeyEvent::new(KeyCode::Char('1'), KeyModifiers::NONE)),
        )
        .await;
        term.draw(|f| self.draw(f))?;
        Ok(())
    }
}
