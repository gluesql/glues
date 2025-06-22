use {
    crate::{
        context::{Context, notebook::ContextState},
        theme,
    },
    ratatui::{
        Frame,
        layout::Rect,
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Padding},
    },
    tui_textarea::TextArea,
};

const NOTE_SYMBOL: &str = "󱇗 ";

pub fn draw(frame: &mut Frame, area: Rect, context: &mut Context) {
    let t = theme::current_theme();
    context.notebook.editor_height = area.height - 2;

    let (title, mut bottom_left) = if let Some(tab_index) = context.notebook.tab_index {
        let mut title = vec![];
        for (i, tab) in context.notebook.tabs.iter().enumerate() {
            let name = format!(" {NOTE_SYMBOL}{} ", tab.note.name.clone());
            let name = if i == tab_index {
                if context.notebook.state.is_editor() {
                    name.fg(t.accent_text).bg(t.accent)
                } else {
                    name.fg(t.accent_text).bg(t.inactive_bg)
                }
            } else {
                name.fg(t.hint)
            };

            title.push(name);
        }

        let title = Line::from(title);
        let mut breadcrumb = vec![];
        let last_index = context.notebook.tabs[tab_index].breadcrumb.len() - 1;

        for (i, name) in context.notebook.tabs[tab_index]
            .breadcrumb
            .iter()
            .enumerate()
        {
            let (color_a, color_b) = if i % 2 == 0 {
                (t.crumb_a, t.crumb_b)
            } else {
                (t.crumb_b, t.crumb_a)
            };

            let name = if i == 0 {
                breadcrumb.push(Span::raw("  󰝰 ").fg(t.crumb_icon).bg(color_a));
                format!("{name} ")
            } else if i == last_index {
                format!(" 󱇗 {name} ")
            } else {
                breadcrumb.push(Span::raw(" 󰝰 ").fg(t.crumb_icon).bg(color_a));
                format!("{name} ")
            };

            breadcrumb.push(Span::raw(name).fg(t.text).bg(color_a));

            if i < last_index {
                breadcrumb.push(Span::raw("").fg(color_a).bg(color_b));
            } else {
                breadcrumb.push(Span::raw("").fg(color_a).bg(t.background));
            }
        }

        (title, breadcrumb)
    } else {
        (
            Line::from("[Editor]".fg(t.inactive_text)),
            vec![Span::default()],
        )
    };

    let (mode, bg) = match context.notebook.state {
        ContextState::EditorNormalMode { .. } => (
            Span::raw(" NORMAL ").fg(t.text).bg(t.background),
            t.background,
        ),
        ContextState::EditorInsertMode => (
            Span::raw(" INSERT ").fg(t.accent_text).bg(t.accent),
            t.accent,
        ),
        ContextState::EditorVisualMode => {
            (Span::raw(" VISUAL ").fg(t.error_text).bg(t.error), t.error)
        }
        _ => (Span::raw("        ").bg(t.surface), t.surface),
    };

    if context.notebook.tab_index.is_some() {
        bottom_left.insert(0, mode);
        bottom_left.insert(1, Span::raw("").fg(bg).bg(t.crumb_a));
    }

    let bottom_left = Line::from(bottom_left);
    let block = Block::new().title(title).title_bottom(bottom_left);
    let block = match (
        context.last_log.as_ref(),
        context.notebook.editors.iter().any(|(_, item)| item.dirty),
    ) {
        (_, true) => block.title_bottom(
            Line::from(vec![
                Span::raw("").fg(t.accent).bg(t.background),
                Span::raw(" 󰔚 Saving... ").fg(t.accent_text).bg(t.accent),
            ])
            .right_aligned(),
        ),
        (Some((log, _)), false) => block.title_bottom(
            Line::from(format!(" {} ", log).fg(t.success_text).bg(t.success)).right_aligned(),
        ),
        (None, false) => block,
    }
    .fg(t.text)
    .bg(t.background)
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
            Style::default().fg(t.accent_text).bg(t.accent),
            Style::default().underlined(),
        ),
        _ => (Style::default(), Style::default()),
    };

    editor.set_cursor_style(cursor_style);
    editor.set_cursor_line_style(cursor_line_style);
    if show_line_number {
        editor.set_line_number_style(Style::default().fg(t.inactive_text));
    } else {
        editor.remove_line_number();
    }

    frame.render_widget(&*editor, area);
}
