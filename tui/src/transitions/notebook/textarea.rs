use tui_textarea::{CursorMove, TextArea};

pub(super) fn cursor_move_forward(editor: &TextArea, n: usize) -> CursorMove {
    let (row, col) = editor.cursor();
    if col + n >= editor.lines()[row].len() {
        CursorMove::End
    } else {
        CursorMove::Jump(row as u16, (col + n) as u16)
    }
}

pub(super) fn cursor_move_back(editor: &TextArea, n: usize) -> CursorMove {
    let (row, col) = editor.cursor();
    if col < n {
        CursorMove::Head
    } else {
        CursorMove::Jump(row as u16, (col - n) as u16)
    }
}

pub(super) fn cursor_move_down(editor: &TextArea, n: usize) -> CursorMove {
    let num_lines = editor.lines().len();
    let (row, col) = editor.cursor();
    if row + n >= num_lines {
        CursorMove::Bottom
    } else {
        CursorMove::Jump((row + n) as u16, col as u16)
    }
}

pub(super) fn cursor_move_up(editor: &TextArea, n: usize) -> CursorMove {
    let (row, col) = editor.cursor();
    if row < n {
        CursorMove::Top
    } else {
        CursorMove::Jump((row - n) as u16, col as u16)
    }
}

pub(super) fn move_cursor_to_line_non_empty_start(editor: &mut TextArea) {
    editor.move_cursor(CursorMove::Head);

    let (row, _) = editor.cursor();
    let is_whitespace_at_first = editor.lines()[row]
        .chars()
        .next()
        .map(|c| c.is_whitespace())
        .unwrap_or(false);
    if is_whitespace_at_first {
        editor.move_cursor(CursorMove::WordForward);
    }
}

pub(super) fn move_cursor_word_end(editor: &mut TextArea, n: usize) {
    for _ in 0..n {
        editor.move_cursor(CursorMove::WordEnd);
    }
}

pub(super) fn move_cursor_word_back(editor: &mut TextArea, n: usize) {
    for _ in 0..n {
        editor.move_cursor(CursorMove::WordBack);
    }
}

pub(super) fn reselect_for_yank(editor: &mut TextArea) {
    let (begin, end) = match editor.selection_range() {
        None => return,
        Some(range) => range,
    };

    editor.cancel_selection();
    editor.move_cursor(CursorMove::Jump(begin.0 as u16, begin.1 as u16));
    editor.start_selection();
    editor.move_cursor(CursorMove::Jump(end.0 as u16, end.1 as u16));
    editor.move_cursor(CursorMove::Forward);
}

pub(super) fn switch_case(editor: &mut TextArea) {
    let yank = editor.yank_text();
    reselect_for_yank(editor);
    editor.cut();

    let changed = editor
        .yank_text()
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect::<String>();

    editor.insert_str(changed);
    editor.set_yank_text(yank);
}
