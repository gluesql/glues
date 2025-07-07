use ratzilla::ratatui::{
    Terminal,
    layout::Alignment,
    widgets::{Block, Paragraph},
};
use ratzilla::{DomBackend, WebRenderer};
use std::io;

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.on_key_event(|_key_event| {
        // handle key events here if needed
    });

    terminal.draw_web(|f| {
        f.render_widget(
            Paragraph::new("Glues running in the browser (press 'q' to quit)")
                .alignment(Alignment::Center)
                .block(Block::bordered().title("Glues")),
            f.area(),
        );
    });

    Ok(())
}
