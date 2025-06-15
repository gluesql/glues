use glues::{App, config, logger};
use insta::assert_snapshot;
use ratatui::{
    Terminal,
    backend::TestBackend,
    crossterm::event::{Event as Input, KeyCode, KeyEvent, KeyModifiers},
};
use std::{fs, path::Path};

fn save_buffer(backend: &TestBackend, name: &str) {
    let dir = Path::new("tui/tests/frames");
    fs::create_dir_all(dir).unwrap();
    let path = dir.join(name);
    fs::write(path, format!("{}", backend)).unwrap();
}

async fn send_key(app: &mut App, terminal: &mut Terminal<TestBackend>, code: KeyCode) {
    let input = Input::Key(KeyEvent::new(code, KeyModifiers::NONE));
    app.handle_input(input).await;
    terminal.draw(|f| app.draw(f)).unwrap();
}

#[tokio::test]
async fn scenario_screenshots() {
    config::init().await;
    logger::init().await;
    let mut app = App::new().await;
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| app.draw(f)).unwrap();
    save_buffer(terminal.backend(), "frame_0_entry.txt");

    send_key(&mut app, &mut terminal, KeyCode::Char('a')).await;
    save_buffer(terminal.backend(), "frame_1_help.txt");

    send_key(&mut app, &mut terminal, KeyCode::Char('x')).await; // close help
    send_key(&mut app, &mut terminal, KeyCode::Char('1')).await; // open memory
    save_buffer(terminal.backend(), "frame_2_notebook.txt");

    send_key(&mut app, &mut terminal, KeyCode::Char('m')).await; // directory actions
    save_buffer(terminal.backend(), "frame_3_dir_actions.txt");

    send_key(&mut app, &mut terminal, KeyCode::Enter).await; // prompt add note
    save_buffer(terminal.backend(), "frame_4_prompt.txt");

    for ch in "Hello".chars() {
        send_key(&mut app, &mut terminal, KeyCode::Char(ch)).await;
    }
    save_buffer(terminal.backend(), "frame_5_typing.txt");

    send_key(&mut app, &mut terminal, KeyCode::Enter).await; // create note
    save_buffer(terminal.backend(), "frame_6_note_added.txt");

    assert_snapshot!("scenario_final", format!("{}", terminal.backend()));
}
