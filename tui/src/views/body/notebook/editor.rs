use {
    crate::{
        color::*,
        context::{notebook::ContextState, Context},
    },
    ratatui::{
        layout::Rect,
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Padding},
        Frame,
    },
    tui_textarea::TextArea,
};

const NOTE_SYMBOL: &str = "󱇗 ";

pub fn draw(frame: &mut Frame, area: Rect, context: &mut Context) {
    context.notebook.editor_height = area.height - 2;

    let (title, breadcrumb) = if let Some(tab_index) = context.notebook.tab_index {
        let mut title = vec![];
        for (i, tab) in context.notebook.tabs.iter().enumerate() {
            let name = format!(" {NOTE_SYMBOL}{} ", tab.note.name.clone());
            let name = if i == tab_index {
                if context.notebook.state.is_editor() {
                    name.fg(WHITE).bg(BLUE)
                } else {
                    name.fg(WHITE).bg(GRAY_DIM)
                }
            } else {
                name.fg(GRAY_MEDIUM)
            };

            title.push(name);
        }

        let title = Line::from(title);
        let breadcrumb = Span::raw(format!(
            " {} ",
            context.notebook.tabs[tab_index].breadcrumb.join("/")
        ))
        .fg(BLACK)
        .bg(GREEN);
        (title, breadcrumb)
    } else {
        (Line::from("[Editor]".fg(GRAY_DIM)), Span::default())
    };

    let mode = match context.notebook.state {
        ContextState::EditorNormalMode { .. } => Span::raw(" NORMAL ").fg(WHITE).bg(BLACK),
        ContextState::EditorInsertMode => Span::raw(" INSERT ").fg(BLACK).bg(YELLOW),
        ContextState::EditorVisualMode => Span::raw(" VISUAL ").fg(WHITE).bg(RED),
        _ => Span::raw("        ").bg(GRAY_DARK),
    };

    let bottom_left = Line::from(vec![mode, breadcrumb]);
    let block = Block::new().title(title).title_bottom(bottom_left);
    let block = match (
        context.last_log.as_ref(),
        context.notebook.editors.iter().any(|(_, item)| item.dirty),
    ) {
        (_, true) => block.title_bottom(
            Line::from(" 󰔚 Saving... ")
                .fg(BLACK)
                .bg(YELLOW)
                .right_aligned(),
        ),
        (Some((log, _)), false) => {
            block.title_bottom(Line::from(format!(" {} ", log).fg(BLACK).bg(GREEN)).right_aligned())
        }
        (None, false) => block,
    }
    .fg(WHITE)
    .bg(GRAY_BLACK)
    .padding(if context.notebook.show_line_number {
        Padding::ZERO
    } else {
        Padding::left(1)
    });

    let show_line_number = context.notebook.show_line_number;
    let state = context.notebook.state;
    let mut editor = TextArea::from("Welcome to Glues :D".lines());
    let editor = if context.notebook.tab_index.is_some() {
        context.notebook.get_editor_mut()
    } else {
        &mut editor
    };

    editor.set_block(block);

    let (cursor_style, cursor_line_style) = match state {
        ContextState::EditorNormalMode { .. }
        | ContextState::EditorInsertMode
        | ContextState::EditorVisualMode => (
            Style::default().fg(WHITE).bg(BLUE),
            Style::default().underlined(),
        ),
        _ => (Style::default(), Style::default()),
    };

    editor.set_cursor_style(cursor_style);
    editor.set_cursor_line_style(cursor_line_style);
    if show_line_number {
        editor.set_line_number_style(Style::default().fg(GRAY_DIM));
    } else {
        editor.remove_line_number();
    }

    frame.render_widget(&*editor, area);
}
