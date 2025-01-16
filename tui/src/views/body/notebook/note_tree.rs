use {
    crate::{
        color::*,
        context::{
            notebook::{ContextState, TreeItem, TreeItemKind},
            NotebookContext,
        },
    },
    ratatui::{
        layout::Rect,
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Borders, HighlightSpacing, List, ListDirection},
        Frame,
    },
};

const CLOSED_SYMBOL: &str = "󰉋 ";
const OPEN_SYMBOL: &str = "󰝰 ";
const NOTE_SYMBOL: &str = "󱇗 ";

pub fn draw(frame: &mut Frame, area: Rect, context: &mut NotebookContext) {
    let note_tree_focused = matches!(
        context.state,
        ContextState::NoteTreeBrowsing | ContextState::NoteTreeNumbering | ContextState::MoveMode
    );
    let title = "[Browser]";
    let title = if note_tree_focused {
        title.fg(SKY_BLUE)
    } else {
        title.fg(GRAY_DIM)
    };
    let block = Block::new().borders(Borders::RIGHT).title(title);
    let inner_area = block.inner(area);

    let tree_items = context.tree_items.iter().map(
        |TreeItem {
             depth,
             target,
             selectable,
             kind,
         }| {
            let line = match kind {
                TreeItemKind::Note { note } => {
                    let pad = depth * 2;
                    Line::from(vec![
                        format!("{:pad$}", "").into(),
                        Span::raw(NOTE_SYMBOL).dim(),
                        Span::raw(&note.name),
                    ])
                }
                TreeItemKind::Directory { directory, opened } => {
                    let pad = depth * 2;
                    let symbol = if *opened { OPEN_SYMBOL } else { CLOSED_SYMBOL };
                    Line::from(vec![
                        format!("{:pad$}", "").into(),
                        Span::raw(symbol).fg(YELLOW),
                        Span::raw(&directory.name),
                    ])
                }
            };

            match (selectable, target) {
                (true, _) => line.fg(WHITE),
                (false, true) => line.fg(MAGENTA),
                (false, false) => line.dim(),
            }
        },
    );

    let list = List::new(tree_items)
        .highlight_style(Style::new().fg(GRAY_WHITE).bg(if note_tree_focused {
            BLUE
        } else {
            GRAY_DARK
        }))
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(block.bg(GRAY_BLACK), area);
    frame.render_stateful_widget(list, inner_area, &mut context.tree_state);
}
