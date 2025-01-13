use {
    crate::context::{
        notebook::{ContextState, TreeItem, TreeItemKind},
        NotebookContext,
    },
    ratatui::{
        layout::Rect,
        style::{Color, Style, Stylize},
        text::Line,
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
        title.light_blue()
    } else {
        title.dark_gray()
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
            match kind {
                TreeItemKind::Note { note } => {
                    let pad = depth * 2;
                    let line = Line::raw(format!("{:pad$}{NOTE_SYMBOL}{}", "", note.name));

                    match (selectable, target) {
                        (true, _) => line,
                        (false, true) => line.light_blue(),
                        (false, false) => line.dim(),
                    }
                }
                TreeItemKind::Directory { directory, opened } => {
                    let pad = depth * 2;
                    let symbol = if *opened { OPEN_SYMBOL } else { CLOSED_SYMBOL };
                    let line = Line::raw(format!("{:pad$}{symbol}{}", "", directory.name));

                    if !selectable {
                        line.dim()
                    } else {
                        line
                    }
                }
            }
        },
    );

    let list = List::new(tree_items)
        .highlight_style(Style::new().fg(Color::White).bg(if note_tree_focused {
            Color::Blue
        } else {
            Color::DarkGray
        }))
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(block, area);
    frame.render_stateful_widget(list, inner_area, &mut context.tree_state);
}
