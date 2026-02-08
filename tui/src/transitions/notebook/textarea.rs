use edtui::{EditorMode, EditorState, Index2, RowIndex, actions::SwitchMode};

/// Creates a selection on the editor spanning from `start` to `end`.
///
/// Since `edtui::Selection` is not publicly constructible (private module),
/// we use `SwitchMode(Visual)` to create a selection at the cursor, then
/// patch the public `start`/`end` fields to the desired range.
pub(super) fn set_selection(editor: &mut EditorState, start: Index2, end: Index2) {
    let saved_mode = editor.mode;
    editor.execute(SwitchMode(EditorMode::Visual));
    if let Some(sel) = &mut editor.selection {
        sel.start = start;
        sel.end = end;
    }
    editor.mode = saved_mode;
}

pub(super) fn switch_case(editor: &mut EditorState) {
    let row = editor.cursor.row;
    let col = editor.cursor.col;

    let Some(ch) = editor.lines.get(Index2::new(row, col)) else {
        return;
    };

    let changed: char = if ch.is_uppercase() {
        ch.to_lowercase().next().unwrap_or(*ch)
    } else {
        ch.to_uppercase().next().unwrap_or(*ch)
    };

    if let Some(cell) = editor.lines.get_mut(Index2::new(row, col)) {
        *cell = changed;
    }
}

pub(super) fn switch_case_selection(editor: &mut EditorState) {
    transform_selection(editor, |c| {
        if c.is_uppercase() {
            c.to_lowercase().next().unwrap_or(c)
        } else {
            c.to_uppercase().next().unwrap_or(c)
        }
    });
}

pub(super) fn transform_selection(editor: &mut EditorState, transform: fn(char) -> char) {
    let selection = match &editor.selection {
        Some(s) => s.clone(),
        None => return,
    };

    let start = selection.start();
    let end = selection.end();

    for row in start.row..=end.row {
        let Some(line) = editor.lines.get(RowIndex::new(row)) else {
            continue;
        };

        let start_col = if row == start.row { start.col } else { 0 };
        let end_col = if row == end.row {
            end.col
        } else {
            line.len().saturating_sub(1)
        };

        for col in start_col..=end_col {
            if let Some(cell) = editor.lines.get_mut(Index2::new(row, col)) {
                *cell = transform(*cell);
            }
        }
    }

    editor.selection = None;
}
