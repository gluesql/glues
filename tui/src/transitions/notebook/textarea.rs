use edtui::{EditorMode, EditorState, Index2, Lines, RowIndex, actions::SwitchMode};

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

// ---------------------------------------------------------------------------
// Unicode-aware word motion
// ---------------------------------------------------------------------------

/// Classify a character into one of three classes: word, whitespace, punctuation.
/// Characters in the same class form a single "word" for motion purposes.
/// Unlike edtui's built-in `CharacterClass`, this uses `char::is_alphanumeric()`
/// (Unicode-aware) instead of `is_ascii_alphanumeric()`, so CJK characters are
/// correctly treated as word characters rather than "Unknown".
fn char_class(ch: &char) -> u8 {
    if ch.is_alphanumeric() || *ch == '_' {
        0 // word
    } else if ch.is_whitespace() {
        1 // whitespace
    } else {
        2 // punctuation / other
    }
}

/// Skip whitespace forward, crossing lines if needed.
/// If whitespace extends to end of the current row, advances to next rows
/// until a non-whitespace character or end of document is reached.
/// Returns `true` if a non-whitespace character was found.
fn skip_ws(lines: &Lines, idx: &mut Index2) -> bool {
    let last_row = lines.len().saturating_sub(1);
    loop {
        if let Some(line) = lines.get(RowIndex::new(idx.row)) {
            for (i, ch) in line.iter().enumerate().skip(idx.col) {
                if !ch.is_whitespace() {
                    idx.col = i;
                    return true;
                }
            }
        }
        // Whitespace to end of row — try next row
        if idx.row >= last_row {
            return false;
        }
        idx.row += 1;
        idx.col = 0;
    }
}

/// Skip whitespace backward on the same row.
fn skip_ws_rev(lines: &Lines, idx: &mut Index2) {
    if let Some(line) = lines.get(RowIndex::new(idx.row)) {
        let skip = line.len().saturating_sub(idx.col + 1);
        for ch in line.iter().rev().skip(skip) {
            if !ch.is_whitespace() {
                break;
            }
            idx.col = idx.col.saturating_sub(1);
        }
    }
}

fn max_col_for_mode(editor: &EditorState) -> usize {
    let len = editor.lines.len_col(editor.cursor.row).unwrap_or(0);
    if editor.mode == EditorMode::Normal {
        len.saturating_sub(1)
    } else {
        len
    }
}

/// Update the visual-mode selection so that `selection.end` tracks the cursor.
/// This mirrors what edtui's internal `set_selection_with_lines` does.
fn update_visual_selection(editor: &mut EditorState) {
    if editor.mode == EditorMode::Visual
        && let Some(sel) = &mut editor.selection
    {
        sel.end = editor.cursor;
    }
}

/// Move the cursor forward by `n` words (vim `w`).
pub(super) fn move_word_forward(editor: &mut EditorState, n: usize) {
    if editor.lines.is_empty() {
        return;
    }
    // clamp column
    let max = max_col_for_mode(editor);
    if editor.cursor.col > max {
        editor.cursor.col = max;
    }

    for _ in 0..n {
        move_word_forward_once(editor);
    }
    update_visual_selection(editor);
}

fn move_word_forward_once(editor: &mut EditorState) {
    let cur = editor.cursor;
    let last_row = editor.lines.len().saturating_sub(1);
    let last_col = editor.lines.len_col(cur.row).unwrap_or(0).saturating_sub(1);
    let is_last_col = cur.col >= last_col;

    match (is_last_col, cur.row >= last_row) {
        (true, true) => return,
        (true, false) => {
            // Jump to next line and skip whitespace
            editor.cursor = Index2::new(cur.row + 1, 0);
            skip_ws(&editor.lines, &mut editor.cursor);
            return;
        }
        _ => {}
    }

    let start_class = match editor.lines.get(cur) {
        Some(ch) => char_class(ch),
        None => return,
    };

    // Advance past characters of the same class
    let mut idx = Index2::new(cur.row, cur.col + 1);
    let line_len = editor.lines.len_col(cur.row).unwrap_or(0);

    while idx.col < line_len {
        match editor.lines.get(idx) {
            Some(ch) if char_class(ch) == start_class => idx.col += 1,
            _ => break,
        }
    }

    if idx.col >= line_len {
        // Word/whitespace extends to end of line — advance to next line
        if cur.row >= last_row {
            editor.cursor = Index2::new(cur.row, last_col);
        } else {
            editor.cursor = Index2::new(cur.row + 1, 0);
            skip_ws(&editor.lines, &mut editor.cursor);
        }
        return;
    }

    editor.cursor = idx;
    // Skip whitespace (may cross lines) to land on the next word
    skip_ws(&editor.lines, &mut editor.cursor);
}

/// Move the cursor backward by `n` words (vim `b`).
pub(super) fn move_word_backward(editor: &mut EditorState, n: usize) {
    if editor.lines.is_empty() {
        return;
    }
    let max = max_col_for_mode(editor);
    if editor.cursor.col > max {
        editor.cursor.col = max;
    }

    for _ in 0..n {
        move_word_backward_once(editor);
    }
    update_visual_selection(editor);
}

fn move_word_backward_once(editor: &mut EditorState) {
    loop {
        let cur = editor.cursor;
        if cur.row == 0 && cur.col == 0 {
            return;
        }
        if cur.col == 0 {
            // Move to end of previous line, then loop to find word start
            let prev_row = cur.row - 1;
            editor.cursor.row = prev_row;
            editor.cursor.col = editor
                .lines
                .len_col(prev_row)
                .unwrap_or(0)
                .saturating_sub(1);
            continue;
        }

        let mut idx = Index2::new(cur.row, cur.col - 1);
        // Skip whitespace backward
        skip_ws_rev(&editor.lines, &mut idx);

        // If we backed up to col 0 and it's still whitespace, cross to previous line
        if let Some(ch) = editor.lines.get(idx)
            && ch.is_whitespace()
            && idx.row > 0
        {
            editor.cursor = idx;
            continue;
        }

        let start_class = match editor.lines.get(idx) {
            Some(ch) => char_class(ch),
            None => {
                editor.cursor = idx;
                return;
            }
        };

        // Walk backward while same class
        loop {
            if idx.col == 0 {
                editor.cursor = idx;
                return;
            }
            let prev = Index2::new(idx.row, idx.col - 1);
            match editor.lines.get(prev) {
                Some(ch) if char_class(ch) == start_class => idx = prev,
                _ => break,
            }
        }

        editor.cursor = idx;
        return;
    }
}

/// Move the cursor forward to the end of the current/next word by `n` times (vim `e`).
pub(super) fn move_word_forward_to_end(editor: &mut EditorState, n: usize) {
    if editor.lines.is_empty() {
        return;
    }
    let max = max_col_for_mode(editor);
    if editor.cursor.col > max {
        editor.cursor.col = max;
    }

    for _ in 0..n {
        move_word_forward_to_end_once(editor);
    }
    update_visual_selection(editor);
}

fn move_word_forward_to_end_once(editor: &mut EditorState) {
    let cur = editor.cursor;
    let last_row = editor.lines.len().saturating_sub(1);
    let last_col = editor.lines.len_col(cur.row).unwrap_or(0).saturating_sub(1);
    let is_last_col = cur.col >= last_col;

    let mut start = match (is_last_col, cur.row >= last_row) {
        (true, true) => return,
        (true, false) => Index2::new(cur.row + 1, 0),
        _ => Index2::new(cur.row, cur.col + 1),
    };

    // Skip whitespace (handles empty rows too since skip_ws crosses lines)
    if !skip_ws(&editor.lines, &mut start) {
        return;
    }

    let start_class = match editor.lines.get(start) {
        Some(ch) => char_class(ch),
        None => return,
    };

    let line_len = editor.lines.len_col(start.row).unwrap_or(0);
    let mut idx = start;

    while idx.col < line_len {
        match editor.lines.get(idx) {
            Some(ch) if char_class(ch) == start_class => {
                editor.cursor = idx;
                if idx.col + 1 >= line_len {
                    break;
                }
                idx.col += 1;
            }
            _ => break,
        }
    }
}

/// Select the inner word under the cursor (for `diw` / `ciw`).
/// Sets `editor.selection` spanning the word, staying on the current line.
pub(super) fn select_inner_word(editor: &mut EditorState) {
    let row = editor.cursor.row;
    let col = editor.cursor.col;

    let Some(line) = editor.lines.get(RowIndex::new(row)) else {
        return;
    };

    let len = line.len();
    if len == 0 || col >= len {
        return;
    }

    let cur_class = char_class(&line[col]);

    // Find start: walk backward while same class
    let mut start_col = col;
    while start_col > 0 && char_class(&line[start_col - 1]) == cur_class {
        start_col -= 1;
    }

    // Find end: walk forward while same class
    let mut end_col = col;
    while end_col + 1 < len && char_class(&line[end_col + 1]) == cur_class {
        end_col += 1;
    }

    set_selection(
        editor,
        Index2::new(row, start_col),
        Index2::new(row, end_col),
    );
}
