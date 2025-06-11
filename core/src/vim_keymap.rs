use crate::{
    transition::VimKeymapKind,
    types::{KeymapGroup, KeymapItem},
};

pub fn keymap(kind: VimKeymapKind) -> Vec<KeymapGroup> {
    match kind {
        VimKeymapKind::NormalIdle => vec![
            KeymapGroup::new(
                "TO INSERT MODE",
                vec![
                    KeymapItem::new("i", "Go to insert mode"),
                    KeymapItem::new("I", "Go to insert mode at the beginning of the line"),
                    KeymapItem::new("o", "Insert a new line below and go to insert mode"),
                    KeymapItem::new("O", "Insert a new line above and go to insert mode"),
                    KeymapItem::new("a", "Move cursor forward and go to insert mode"),
                    KeymapItem::new(
                        "A",
                        "Move cursor to the end of the line and go to insert mode",
                    ),
                    KeymapItem::new("s", "Delete character and go to insert mode"),
                    KeymapItem::new("S", "Delete line and go to insert mode"),
                ],
            ),
            KeymapGroup::new(
                "TO OTHER MODES",
                vec![
                    KeymapItem::new("c", "Go to change mode (prepare to edit text)"),
                    KeymapItem::new("v", "Go to visual mode (select text to edit or copy)"),
                    KeymapItem::new("g", "Go to gateway mode (access extended commands)"),
                    KeymapItem::new("y", "Go to yank mode (prepare to copy text)"),
                    KeymapItem::new("d", "Go to delete mode (prepare to delete text)"),
                    KeymapItem::new("z", "Go to scroll mode (adjust viewport)"),
                    KeymapItem::new(
                        "1-9",
                        "Go to numbering mode (repeat or extend actions with numbers)",
                    ),
                ],
            ),
            KeymapGroup::new(
                "MOVE CURSOR",
                vec![
                    KeymapItem::new("h", "Move cursor left"),
                    KeymapItem::new("j", "Move cursor down"),
                    KeymapItem::new("k", "Move cursor up"),
                    KeymapItem::new("l", "Move cursor right"),
                    KeymapItem::new("w", "Move cursor to the start of the next word"),
                    KeymapItem::new("e", "Move cursor to the end of the current word"),
                    KeymapItem::new("b", "Move cursor to the start of the previous word"),
                    KeymapItem::new("0", "Move cursor to the start of the line"),
                    KeymapItem::new("$", "Move cursor to the end of the line"),
                    KeymapItem::new(
                        "^",
                        "Move cursor to the first non-blank character of the line",
                    ),
                    KeymapItem::new("G", "Move cursor to the end of the file"),
                ],
            ),
            KeymapGroup::new(
                "EDIT TEXT",
                vec![
                    KeymapItem::new("~", "Toggle the case of the current character"),
                    KeymapItem::new("x", "Delete character under the cursor"),
                    KeymapItem::new("u", "Undo the last change"),
                    KeymapItem::new("Ctrl+r", "Redo the last undone change"),
                ],
            ),
        ],
        VimKeymapKind::NormalNumbering => vec![
            KeymapGroup::new(
                "EXTENDING NUMBERING MODE",
                vec![KeymapItem::new(
                    "0-9",
                    "Append additional digits to extend the current command",
                )],
            ),
            KeymapGroup::new(
                "TO INSERT MODE",
                vec![
                    KeymapItem::new(
                        "s",
                        "Delete specified number of characters and go to insert mode",
                    ),
                    KeymapItem::new("S", "Delete the entire line and go to insert mode"),
                ],
            ),
            KeymapGroup::new(
                "TO OTHER MODES",
                vec![
                    KeymapItem::new(
                        "c",
                        "Go to change mode with repeat count (prepare to edit text)",
                    ),
                    KeymapItem::new(
                        "y",
                        "Go to yank mode with repeat count (prepare to copy text)",
                    ),
                    KeymapItem::new(
                        "d",
                        "Go to delete mode with repeat count (prepare to delete text)",
                    ),
                ],
            ),
            KeymapGroup::new(
                "MOVE CURSOR AND RETURN TO NORMAL MODE",
                vec![
                    KeymapItem::new("h", "Move cursor left by the specified number of times"),
                    KeymapItem::new("j", "Move cursor down by the specified number of times"),
                    KeymapItem::new("k", "Move cursor up by the specified number of times"),
                    KeymapItem::new("l", "Move cursor right by the specified number of times"),
                    KeymapItem::new(
                        "w",
                        "Move cursor to the start of the next word, repeated by the specified number",
                    ),
                    KeymapItem::new(
                        "e",
                        "Move cursor to the end of the next word, repeated by the specified number",
                    ),
                    KeymapItem::new(
                        "b",
                        "Move cursor to the start of the previous word, repeated by the specified number",
                    ),
                    KeymapItem::new("G", "Move cursor to the specified line number"),
                ],
            ),
            KeymapGroup::new(
                "EDIT TEXT AND RETURN TO NORMAL MODE",
                vec![KeymapItem::new(
                    "x",
                    "Delete specified number of characters and return to normal mode",
                )],
            ),
        ],
        VimKeymapKind::NormalDelete => vec![
            KeymapGroup::new(
                "TO NUMBERING MODE",
                vec![KeymapItem::new("1-9", "Go to delete numbering mode")],
            ),
            KeymapGroup::new(
                "TO DELETE INSIDE MODE",
                vec![KeymapItem::new("i", "Go to delete inside mode")],
            ),
            KeymapGroup::new(
                "DELETE TEXT",
                vec![
                    KeymapItem::new("d", "Delete the specified number of lines"),
                    KeymapItem::new("j", "Delete the current and following lines"),
                    KeymapItem::new("k", "Delete the current and previous lines"),
                    KeymapItem::new(
                        "e",
                        "Delete the word from the cursor to the end of the current word",
                    ),
                    KeymapItem::new("b", "Delete the word before the cursor"),
                    KeymapItem::new("0", "Delete to the beginning of the line"),
                    KeymapItem::new(
                        "$",
                        "Delete to the end of the line, repeated by the specified number",
                    ),
                    KeymapItem::new("h", "Delete the specified number of characters to the left"),
                    KeymapItem::new(
                        "l",
                        "Delete the specified number of characters to the right",
                    ),
                ],
            ),
        ],
        VimKeymapKind::NormalDelete2 => vec![
            KeymapGroup::new(
                "EXTENDING NUMBERING MODE",
                vec![KeymapItem::new(
                    "0-9",
                    "Append additional digits to extend the current command",
                )],
            ),
            KeymapGroup::new(
                "TO DELETE INSIDE MODE",
                vec![KeymapItem::new("i", "Go to delete inside mode")],
            ),
            KeymapGroup::new(
                "DELETE TEXT",
                vec![
                    KeymapItem::new("d", "Delete the specified number of lines"),
                    KeymapItem::new("j", "Delete the current and following lines"),
                    KeymapItem::new("k", "Delete the current and previous lines"),
                    KeymapItem::new(
                        "e",
                        "Delete the word from the cursor to the end of the current word",
                    ),
                    KeymapItem::new("b", "Delete the word before the cursor"),
                    KeymapItem::new(
                        "$",
                        "Delete to the end of the line, repeated by the specified number",
                    ),
                    KeymapItem::new("h", "Delete the specified number of characters to the left"),
                    KeymapItem::new(
                        "l",
                        "Delete the specified number of characters to the right",
                    ),
                ],
            ),
        ],
        VimKeymapKind::NormalChange => vec![
            KeymapGroup::new(
                "TO CHANGE INSIDE MODE",
                vec![KeymapItem::new("i", "Go to change inside mode")],
            ),
            KeymapGroup::new(
                "CHANGE TEXT AND GO TO INSERT MODE",
                vec![
                    KeymapItem::new("c", "Delete the specified number of lines"),
                    KeymapItem::new(
                        "e/w",
                        "Delete to the end of the word by the specified number of times",
                    ),
                    KeymapItem::new(
                        "b",
                        "Delete to the start of the previous word, repeated by the specified number",
                    ),
                    KeymapItem::new("0", "Delete to the beginning of the line"),
                    KeymapItem::new(
                        "$",
                        "Delete to the end of the line, repeated by the specified number",
                    ),
                ],
            ),
        ],
        VimKeymapKind::NormalChange2 => vec![
            KeymapGroup::new(
                "EXTENDING NUMBERING MODE",
                vec![KeymapItem::new(
                    "0-9",
                    "Append additional digits to extend the current command",
                )],
            ),
            KeymapGroup::new(
                "TO CHANGE INSIDE MODE",
                vec![KeymapItem::new("i", "Go to change inside mode")],
            ),
            KeymapGroup::new(
                "CHANGE TEXT AND GO TO INSERT MODE",
                vec![
                    KeymapItem::new("c", "Delete the specified number of lines"),
                    KeymapItem::new(
                        "e/w",
                        "Delete to the end of the word by the specified number of times",
                    ),
                    KeymapItem::new(
                        "b",
                        "Delete to the start of the previous word, repeated by the specified number",
                    ),
                    KeymapItem::new(
                        "$",
                        "Delete to the end of the line, repeated by the specified number",
                    ),
                ],
            ),
        ],
        VimKeymapKind::VisualIdle => vec![
            KeymapGroup::new(
                "MOVE CURSOR",
                vec![
                    KeymapItem::new("h", "Move cursor left"),
                    KeymapItem::new("j", "Move cursor down"),
                    KeymapItem::new("k", "Move cursor up"),
                    KeymapItem::new("l", "Move cursor right"),
                    KeymapItem::new("w", "Move cursor to the start of the next word"),
                    KeymapItem::new("e", "Move cursor to the end of the next word"),
                    KeymapItem::new("b", "Move cursor to the start of the previous word"),
                    KeymapItem::new("0", "Move cursor to the start of the line"),
                    KeymapItem::new("$", "Move cursor to the end of the line"),
                    KeymapItem::new(
                        "^",
                        "Move cursor to the first non-blank character of the line",
                    ),
                    KeymapItem::new("G", "Move cursor to the end of the file"),
                ],
            ),
            KeymapGroup::new(
                "TO INSERT MODE",
                vec![KeymapItem::new(
                    "s/S",
                    "Substitute selected text and go to insert mode",
                )],
            ),
            KeymapGroup::new(
                "TO EXTENDED MODES",
                vec![
                    KeymapItem::new("g", "Go to gateway mode for additional commands"),
                    KeymapItem::new("1-9", "Specify repeat count for subsequent actions"),
                ],
            ),
            KeymapGroup::new(
                "EDIT TEXT AND RETURN TO NORMAL MODE",
                vec![
                    KeymapItem::new("d/x", "Delete selected text"),
                    KeymapItem::new("y", "Yank (copy) selected text"),
                    KeymapItem::new("~", "Toggle the case of the select text"),
                ],
            ),
        ],
        VimKeymapKind::VisualNumbering => vec![
            KeymapGroup::new(
                "EXTENDING NUMBERING MODE",
                vec![KeymapItem::new(
                    "0-9",
                    "Append additional digits to extend the current command",
                )],
            ),
            KeymapGroup::new(
                "MOVE CURSOR",
                vec![
                    KeymapItem::new("h", "Move cursor left by the specified number of times"),
                    KeymapItem::new("j", "Move cursor down by the specified number of times"),
                    KeymapItem::new("k", "Move cursor up by the specified number of times"),
                    KeymapItem::new("l", "Move cursor right by the specified number of times"),
                    KeymapItem::new(
                        "w",
                        "Move cursor to the start of the next word, repeated by the specified number",
                    ),
                    KeymapItem::new(
                        "e",
                        "Move cursor to the end of the next word, repeated by the specified number",
                    ),
                    KeymapItem::new(
                        "b",
                        "Move cursor to the start of the previous word, repeated by the specified number",
                    ),
                    KeymapItem::new("G", "Move cursor to the specified line number"),
                ],
            ),
        ],
    }
}
