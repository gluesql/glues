use {
    crate::theme::THEME,
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Style, Stylize},
        text::Line,
        widgets::{Block, Clear, Padding, Paragraph, Wrap},
    },
};

pub fn draw(frame: &mut Frame) {
    let [area] = Layout::horizontal([Length(80)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(27)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::bordered()
        .fg(THEME.text)
        .bg(THEME.surface)
        .padding(Padding::new(2, 2, 1, 1))
        .title(Line::from("Editor Keymap").centered());

    let inner_area = block.inner(area);
    let [message_area, control_area] = Layout::vertical([Length(22), Length(1)])
        .flex(Flex::SpaceBetween)
        .areas(inner_area);

    let message = "
| Mappings                 | Description                                |
|--------------------------|--------------------------------------------|
| `Backspace`              | Delete one character before cursor         |
| `Ctrl+D`, `Delete`       | Delete one character next to cursor        |
| `Enter`, `Ctrl+J`        | Insert newline                             |
| `Ctrl+K`                 | Delete from cursor until the end of line   |
| `Ctrl+U`                 | Delete from cursor until the head of line  |
| `Ctrl+R`                 | Redo                                       |
| `Ctrl+Y`                 | Paste yanked text                          |
| `Ctrl+F`, `→`            | Move cursor forward by one character       |
| `Ctrl+B`, `←`            | Move cursor backward by one character      |
| `Ctrl+P`, `↑`            | Move cursor up by one line                 |
| `Ctrl+N`, `↓`            | Move cursor down by one line               |
| `Ctrl+A`, `Home`         | Move cursor to the head of line            |
| `Ctrl+E`, `End`          | Move cursor to the end of line             |
| `Alt+<`                  | Move cursor to top of lines                |
| `Alt+>`                  | Move cursor to bottom of lines             |
"
    .lines()
    .map(Line::from)
    .collect::<Vec<_>>();

    let paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);
    let control = Line::from("Press any key to close".fg(THEME.hint)).centered();

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, message_area);
    frame.render_widget(control, control_area);
}
