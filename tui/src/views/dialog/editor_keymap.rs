use {
    crate::color::*,
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Style, Stylize},
        text::Line,
        widgets::{Block, Clear, Padding, Paragraph, Wrap},
    },
};

pub fn draw(frame: &mut Frame) {
    let [area] = Layout::horizontal([Length(100)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(38)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::bordered()
        .fg(WHITE)
        .bg(GRAY_DARK)
        .padding(Padding::new(2, 2, 1, 1))
        .title("Editor Keymap")
        .title_alignment(Alignment::Center);

    let inner_area = block.inner(area);
    let [message_area, control_area] = Layout::vertical([Length(33), Length(1)])
        .flex(Flex::SpaceBetween)
        .areas(inner_area);

    let message = "
| Mappings                                      | Description                                |
|-----------------------------------------------|--------------------------------------------|
| `Backspace`                                   | Delete one character before cursor         |
| `Ctrl+D`, `Delete`                            | Delete one character next to cursor        |
| `Ctrl+M`, `Enter`                             | Insert newline                             |
| `Ctrl+K`                                      | Delete from cursor until the end of line   |
| `Ctrl+J`                                      | Delete from cursor until the head of line  |
| `Ctrl+W`, `Alt+H`, `Alt+Backspace`            | Delete one word before cursor              |
| `Alt+D`, `Alt+Delete`                         | Delete one word next to cursor             |
| `Ctrl+U`                                      | Undo                                       |
| `Ctrl+R`                                      | Redo                                       |
| `Ctrl+C`, `Copy`                              | Copy selected text                         |
| `Ctrl+X`, `Cut`                               | Cut selected text                          |
| `Ctrl+Y`, `Paste`                             | Paste yanked text                          |
| `Ctrl+F`, `→`                                 | Move cursor forward by one character       |
| `Ctrl+B`, `←`                                 | Move cursor backward by one character      |
| `Ctrl+P`, `↑`                                 | Move cursor up by one line                 |
| `Ctrl+N`, `↓`                                 | Move cursor down by one line               |
| `Alt+F`, `Ctrl+→`                             | Move cursor forward by word                |
| `Atl+B`, `Ctrl+←`                             | Move cursor backward by word               |
| `Alt+]`, `Alt+P`, `Ctrl+↑`                    | Move cursor up by paragraph                |
| `Alt+[`, `Alt+N`, `Ctrl+↓`                    | Move cursor down by paragraph              |
| `Ctrl+E`, `End`, `Ctrl+Alt+F`, `Ctrl+Alt+→`   | Move cursor to the end of line             |
| `Ctrl+A`, `Home`, `Ctrl+Alt+B`, `Ctrl+Alt+←`  | Move cursor to the head of line            |
| `Alt+<`, `Ctrl+Alt+P`, `Ctrl+Alt+↑`           | Move cursor to top of lines                |
| `Alt+>`, `Ctrl+Alt+N`, `Ctrl+Alt+↓`           | Move cursor to bottom of lines             |
| `Ctrl+V`, `PageDown`                          | Scroll down by page                        |
| `Alt+V`, `PageUp`                             | Scroll up by page                          |

Thanks to tui-textarea
"
    .lines()
    .map(Line::from)
    .collect::<Vec<_>>();

    let paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);
    let control = Line::from("Press any key to close".fg(GRAY_MEDIUM)).centered();

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, message_area);
    frame.render_widget(control, control_area);
}
