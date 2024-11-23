use {
    glues_core::transition::VimKeymapKind,
    ratatui::{
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Style, Stylize},
        text::Line,
        widgets::{Block, Clear, Padding, Paragraph, Wrap},
        Frame,
    },
};

pub fn draw(frame: &mut Frame, keymap_kind: VimKeymapKind) {
    let (title, message) = match keymap_kind {
        VimKeymapKind::NormalIdle => (
            "VIM NORMAL MODE KEYMAP",
            vec![
                Line::from("TO INSERT MODE".white().on_dark_gray()),
                Line::raw("[i] Go to insert mode"),
                Line::raw("[I] Go to insert mode at the beginning of the line"),
                Line::raw("[o] Insert a new line below and go to insert mode"),
                Line::raw("[O] Insert a new line above and go to insert mode"),
                Line::raw("[a] Move cursor forward and go to insert mode"),
                Line::raw("[A] Move cursor to the end of the line and go to insert mode"),
                Line::raw("[s] Delete character and go to insert mode"),
                Line::raw("[S] Delete line and go to insert mode"),
                Line::raw(""),
                Line::from("TO OTHER MODES".white().on_dark_gray()),
                Line::raw("[c] Go to change mode (prepare to edit text)"),
                Line::raw("[v] Go to visual mode (select text to edit or copy)"),
                Line::raw("[g] Go to gateway mode (access extended commands)"),
                Line::raw("[y] Go to yank mode (prepare to copy text)"),
                Line::raw("[d] Go to delete mode (prepare to delete text)"),
                Line::raw("[1-9] Go to numbering mode (repeat or extend actions with numbers)"),
                Line::raw(""),
                Line::from("MOVE CURSOR".white().on_dark_gray()),
                Line::raw("[h] Move cursor left"),
                Line::raw("[j] Move cursor down"),
                Line::raw("[k] Move cursor up"),
                Line::raw("[l] Move cursor right"),
                Line::raw("[w] Move cursor to the start of the next word"),
                Line::raw("[e] Move cursor to the end of the next word"),
                Line::raw("[b] Move cursor to the start of the previous word"),
                Line::raw("[0] Move cursor to the start of the line"),
                Line::raw("[$] Move cursor to the end of the line"),
                Line::raw("[^] Move cursor to the first non-blank character of the line"),
                Line::raw("[G] Move cursor to the end of the file"),
                Line::raw(""),
                Line::from("EDIT TEXT".white().on_dark_gray()),
                Line::raw("[x] Delete character under the cursor"),
                Line::raw("[u] Undo the last change"),
                Line::raw("[Ctrl+r] Redo the last undone change"),
            ],
        ),
        VimKeymapKind::NormalNumbering => ("VIM NORMAL MODE KEYMAP - NUMBERING", vec![
            Line::from("EXTENDING NUMBERING MODE".white().on_dark_gray()),
            Line::raw("[0-9] Append additional digits to extend the current command"),
            Line::raw(""),
            Line::from("TO INSERT MODE".white().on_dark_gray()),
            Line::raw("[s] Delete specified number of characters and go to insert mode"),
            Line::raw("[S] Delete the entire line and go to insert mode"),
            Line::raw(""),
            Line::from("TO OTHER MODES".white().on_dark_gray()),
            Line::raw("[c] Go to change mode with repeat count (prepare to edit text)"),
            Line::raw("[y] Go to yank mode with repeat count (prepare to copy text)"),
            Line::raw("[d] Go to delete mode with repeat count (prepare to delete text)"),
            Line::raw(""),
            Line::from("MOVE CURSOR AND RETURN TO NORMAL MODE".white().on_dark_gray()),
            Line::raw("[h] Move cursor left by the specified number of times"),
            Line::raw("[j] Move cursor down by the specified number of times"),
            Line::raw("[k] Move cursor up by the specified number of times"),
            Line::raw("[l] Move cursor right by the specified number of times"),
            Line::raw("[w] Move cursor to the start of the next word, repeated by the specified number"),
            Line::raw("[e] Move cursor to the end of the next word, repeated by the specified number"),
            Line::raw("[b] Move cursor to the start of the previous word, repeated by the specified number"),
            Line::raw("[G] Move cursor to the specified line number"),
            Line::raw(""),
            Line::from("EDIT TEXT AND RETURN TO NORMAL MODE".white().on_dark_gray()),
            Line::raw("[x] Delete specified number of characters and return to normal mode"),
        ]),
        VimKeymapKind::NormalDelete => ("VIM NORMAL MODE KEYMAP - DELETE", vec![
            Line::from("TO NUMBERING MODE".white().on_dark_gray()),
            Line::raw("[1-9] Go to delete numbering mode"),
            Line::raw(""),
            Line::from("TO DELETE INSIDE MODE".white().on_dark_gray()),
            Line::raw("[i] Go to delete inside mode"),
            Line::raw(""),
            Line::from("DELETE TEXT".white().on_dark_gray()),
            Line::raw("[d] Delete the specified number of lines"),
            Line::raw("[de] Delete the word from the cursor to the end of the current word."),
            Line::raw("[db] Delete the word before the cursor."),
            Line::raw("[0] Delete to the beginning of the line"),
            Line::raw("[$] Delete to the end of the line, repeated by the specified number"),
        ]),
        VimKeymapKind::NormalDelete2 => ("VIM NORMAL MODE KEYMAP - DELETE NUMBERING", vec![
            Line::from("EXTENDING NUMBERING MODE".white().on_dark_gray()),
            Line::raw("[0-9] Append additional digits to extend the current command"),
            Line::raw(""),
            Line::from("TO DELETE INSIDE MODE".white().on_dark_gray()),
            Line::raw("[i] Go to delete inside mode"),
            Line::raw(""),
            Line::from("DELETE TEXT".white().on_dark_gray()),
            Line::raw("[d] Delete the specified number of lines"),
        ]),
        VimKeymapKind::NormalChange => ("VIM NORMAL MODE KEYMAP - CHANGE", vec![
            Line::from("TO CHANGE INSIDE MODE".white().on_dark_gray()),
            Line::raw("[i] Go to change inside mode"),
            Line::raw(""),
            Line::from("CHANGE TEXT AND GO TO INSERT MODE".white().on_dark_gray()),
            Line::raw("[c] Delete the specified number of lines"),
            Line::from(vec![
                "[e] ".into(),
                "or ".dark_gray(),
                "[w] Delete to the end of the word by the specified number of times".into(),
            ]),
            Line::raw("[b] Delete to the start of the previous word, repeated by the specified number"),
            Line::raw("[0] Delete to the beginning of the line"),
            Line::raw("[$] Delete to the end of the line, repeated by the specified number"),
        ]),
        VimKeymapKind::NormalChange2 => ("VIM NORMAL MODE KEYMAP - CHANGE NUMBERING", vec![
            Line::from("EXTENDING NUMBERING MODE".white().on_dark_gray()),
            Line::raw("[0-9] Append additional digits to extend the current command"),
            Line::raw(""),
            Line::from("TO CHANGE INSIDE MODE".white().on_dark_gray()),
            Line::raw("[i] Go to change inside mode"),
            Line::raw(""),
            Line::from("CHANGE TEXT AND GO TO INSERT MODE".white().on_dark_gray()),
            Line::raw("[c] Delete the specified number of lines"),
            Line::from(vec![
                "[e] ".into(),
                "or ".dark_gray(),
                "[w] Delete to the end of the word by the specified number of times".into(),
            ]),
            Line::raw("[b] Delete to the start of the previous word, repeated by the specified number"),
            Line::raw("[$] Delete to the end of the line, repeated by the specified number"),
        ]),
        VimKeymapKind::VisualIdle => ("VIM VISUAL MODE KEYMAP", vec![
            Line::from("MOVE CURSOR".white().on_dark_gray()),
            Line::raw("[h] Move cursor left"),
            Line::raw("[j] Move cursor down"),
            Line::raw("[k] Move cursor up"),
            Line::raw("[l] Move cursor right"),
            Line::raw("[w] Move cursor to the start of the next word"),
            Line::raw("[e] Move cursor to the end of the next word"),
            Line::raw("[b] Move cursor to the start of the previous word"),
            Line::raw("[0] Move cursor to the start of the line"),
            Line::raw("[$] Move cursor to the end of the line"),
            Line::raw("[^] Move cursor to the first non-blank character of the line"),
            Line::raw("[G] Move cursor to the end of the file"),
            Line::raw(""),
            Line::from("TO INSERT MODE".white().on_dark_gray()),
            Line::from(vec![
                "[s] ".into(),
                "or ".dark_gray(),
                "[S] Substitute selected text and go to insert mode".into(),
            ]),
            Line::raw(""),
            Line::from("TO EXTENDED MODES".white().on_dark_gray()),
            Line::raw("[g] Go to gateway mode for additional commands"),
            Line::raw("[1-9] Specify repeat count for subsequent actions"),
            Line::raw(""),
            Line::from("EDIT TEXT AND RETURN TO NORMAL MODE".white().on_dark_gray()),
            Line::from(vec![
                "[d] ".into(),
                "or ".dark_gray(),
                "[x] Delete selected text".into(),
            ]),
            Line::raw("[y] Yank (copy) selected text"),
        ]),
        VimKeymapKind::VisualNumbering => ("VIM VISUAL MODE KEYMAP - NUMBERING", vec![
            Line::from("EXTENDING NUMBERING MODE".white().on_dark_gray()),
            Line::raw("[0-9] Append additional digits to extend the current command"),
            Line::raw(""),
            Line::from("MOVE CURSOR".white().on_dark_gray()),
            Line::raw("[h] Move cursor left by the specified number of times"),
            Line::raw("[j] Move cursor down by the specified number of times"),
            Line::raw("[k] Move cursor up by the specified number of times"),
            Line::raw("[l] Move cursor right by the specified number of times"),
            Line::raw("[w] Move cursor to the start of the next word, repeated by the specified number"),
            Line::raw("[e] Move cursor to the end of the next word, repeated by the specified number"),
            Line::raw("[b] Move cursor to the start of the previous word, repeated by the specified number"),
            Line::raw("[G] Move cursor to the specified line number"),
        ]),
    };
    let height = message.len() as u16 + 7;

    let [area] = Layout::horizontal([Length(90)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(height)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::bordered()
        .padding(Padding::new(2, 2, 1, 1))
        .title(title.white().on_dark_gray())
        .title_alignment(Alignment::Center);

    let inner_area = block.inner(area);
    let [message_area, control_area] = Layout::vertical([Length(height - 5), Length(1)])
        .flex(Flex::SpaceBetween)
        .areas(inner_area);
    let paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);
    let control = Line::from("Press any key to close".dark_gray()).centered();

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, message_area);
    frame.render_widget(control, control_area);
}
