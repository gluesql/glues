use {
    crate::context::{
        notebook::{ContextState, TreeItem},
        NotebookContext,
    },
    ratatui::{
        layout::Rect,
        style::{Color, Style},
        text::Line,
        widgets::{Block, HighlightSpacing, List, ListDirection},
        Frame,
    },
};

const CLOSED_SYMBOL: &str = "▶ ";
const OPEN_SYMBOL: &str = "▼ ";

pub fn draw(frame: &mut Frame, area: Rect, context: &mut NotebookContext) {
    let block = Block::bordered().title("[Browser]");
    let inner_area = block.inner(area);

    let tree_items = context.tree_items.iter().map(|item| match item {
        TreeItem::Note { value, depth } => {
            let pad = depth * 2 + 2;
            Line::raw(format!("{:pad$}{}", "", value.name))
        }
        TreeItem::Directory {
            value,
            depth,
            opened,
        } => {
            let pad = depth * 2;
            let symbol = if *opened { OPEN_SYMBOL } else { CLOSED_SYMBOL };
            Line::raw(format!("{:pad$}{symbol}{}", "", value.name))
        }
    });

    let list = List::new(tree_items)
        .highlight_style(Style::new().fg(Color::White).bg(match context.state {
            ContextState::NoteTreeBrowsing => Color::Blue,
            _ => Color::DarkGray,
        }))
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(block, area);
    frame.render_stateful_widget(list, inner_area, &mut context.tree_state);
}
