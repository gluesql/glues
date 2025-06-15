use glues::App;
use ratatui::{Terminal, backend::TestBackend};
use std::fs;

fn save_buffer(backend: &TestBackend, name: &str) {
    let content = format!("{}", backend);
    fs::write(name, content).unwrap();
}

#[tokio::test]
async fn take_screenshots() {
    let mut app = App::new().await;
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| app.draw(f)).unwrap();
    let backend = terminal.backend();
    save_buffer(backend, "frame_init.txt");

    app.context_mut().help = true;
    terminal.draw(|f| app.draw(f)).unwrap();
    let backend = terminal.backend();
    save_buffer(backend, "frame_help.txt");
}
