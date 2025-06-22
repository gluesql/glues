use {
    crate::{
        context::{
            NotebookContext,
            notebook::{ContextState, TreeItem, TreeItemKind},
        },
        theme,
    },
    ratatui::{
        Frame,
        layout::Rect,
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListDirection},
    },
};

const CLOSED_SYMBOL: &str = "󰉋 ";
const OPEN_SYMBOL: &str = "󰝰 ";
const NOTE_SYMBOL: &str = "󱇗 ";

pub fn draw(frame: &mut Frame, area: Rect, context: &mut NotebookContext) {
    let t = theme::current_theme();
    let note_tree_focused = matches!(
        context.state,
        ContextState::NoteTreeBrowsing
            | ContextState::NoteTreeNumbering
            | ContextState::NoteTreeGateway
            | ContextState::MoveMode
    );
    let title = "[Browser]";
    let title = if note_tree_focused {
        title.fg(t.highlight)
    } else {
        title.fg(t.inactive_text)
    };
    let block = Block::new()
        .borders(Borders::RIGHT)
        .border_type(BorderType::QuadrantOutside)
        .fg(t.hint)
        .title(title);
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
                        Span::raw(symbol).fg(t.crumb_icon),
                        Span::raw(&directory.name),
                    ])
                }
            };

            match (selectable, target) {
                (true, _) => line.fg(t.text),
                (false, true) => line.fg(t.target),
                (false, false) => line.dim(),
            }
        },
    );

    let highlight_style = Style::default()
        .bg(if note_tree_focused {
            t.accent
        } else {
            t.surface
        })
        .fg(if note_tree_focused {
            t.accent_text
        } else {
            t.text
        });

    let list = List::new(tree_items)
        .highlight_style(highlight_style)
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(block.bg(t.background), area);
    frame.render_stateful_widget(list, inner_area, &mut context.tree_state);
}
