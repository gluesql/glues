use {
    crate::{
        context::{
            Context,
            notebook::{ContextState, ScrollAnchor, ScrollRequest},
        },
        theme::{THEME, current_theme_id, syntect_theme_name},
    },
    edtui::{EditorState, EditorTheme, EditorView, Index2, LineNumbers, Lines, SyntaxHighlighter},
    ratatui::{
        Frame,
        buffer::Buffer,
        layout::Rect,
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Padding, Widget},
    },
};

const NOTE_SYMBOL: &str = "󱇗 ";
const SAMPLE_NOTE: &str = r#"Welcome to Glues!

Press `?` to see keymaps and shortcuts.
Press `m` in the note tree to create notes or directories.
Press `Enter` on a note to open it and start writing.

GitHub: https://github.com/gluesql/glues"#;

pub fn draw(frame: &mut Frame, area: Rect, context: &mut Context) {
    context.notebook.editor_height = area.height.saturating_sub(2);

    let block = build_block(context);
    let show_line_number = context.notebook.show_line_number;
    let state = context.notebook.state;

    let cursor_style = match state {
        ContextState::EditorNormalMode { .. }
        | ContextState::EditorInsertMode
        | ContextState::EditorVisualMode => Style::default().fg(THEME.accent_text).bg(THEME.accent),
        _ => Style::default(),
    };

    let line_numbers = if show_line_number {
        LineNumbers::Absolute
    } else {
        LineNumbers::None
    };

    let theme = EditorTheme::default()
        .base(Style::default().fg(THEME.text).bg(THEME.background))
        .block(block)
        .cursor_style(cursor_style)
        .selection_style(Style::default().fg(THEME.accent_text).bg(THEME.accent))
        .line_numbers_style(
            Style::default()
                .fg(THEME.inactive_text)
                .bg(THEME.background),
        )
        .hide_status_line();

    let show_syntax = context.notebook.show_syntax_highlight;
    let extension: String = context
        .notebook
        .tab_index
        .and_then(|i| context.notebook.tabs[i].note.name.rsplit_once('.'))
        .map(|(_, ext)| ext.to_owned())
        .unwrap_or_else(|| "md".to_owned());
    let new_highlighter = || {
        if show_syntax {
            let syntax_theme = syntect_theme_name(current_theme_id());
            SyntaxHighlighter::new(syntax_theme, &extension)
                .or_else(|_| SyntaxHighlighter::new(syntax_theme, "md"))
                .ok()
        } else {
            None
        }
    };

    if context.notebook.tab_index.is_some() {
        let scroll_shift = prepare_scroll_viewport(context, area);

        let editor = context.notebook.get_editor_mut();
        EditorView::new(editor)
            .theme(theme)
            .syntax_highlighter(new_highlighter())
            .wrap(true)
            .line_numbers(line_numbers)
            .render(area, frame.buffer_mut());

        if scroll_shift > 0 {
            apply_scroll_shift(frame.buffer_mut(), area, scroll_shift);
        }
    } else {
        let mut sample_state = EditorState::new(Lines::from(SAMPLE_NOTE));
        let theme = theme.hide_cursor();
        EditorView::new(&mut sample_state)
            .theme(theme)
            .syntax_highlighter(new_highlighter())
            .wrap(true)
            .line_numbers(line_numbers)
            .render(area, frame.buffer_mut());
    };
}

fn build_block(context: &Context) -> Block<'static> {
    let (title, mut bottom_left) = if let Some(tab_index) = context.notebook.tab_index {
        let mut title = vec![];
        for (i, tab) in context.notebook.tabs.iter().enumerate() {
            let name = format!(" {NOTE_SYMBOL}{} ", tab.note.name);
            let name = if i == tab_index {
                if context.notebook.state.is_editor() {
                    name.fg(THEME.accent_text).bg(THEME.accent)
                } else {
                    name.fg(THEME.accent_text).bg(THEME.inactive_bg)
                }
            } else {
                name.fg(THEME.hint)
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
                (THEME.crumb_a, THEME.crumb_b)
            } else {
                (THEME.crumb_b, THEME.crumb_a)
            };

            let name = if i == 0 {
                breadcrumb.push(Span::raw("  󰝰 ").fg(THEME.crumb_icon).bg(color_a));
                format!("{name} ")
            } else if i == last_index {
                format!(" 󱇗 {name} ")
            } else {
                breadcrumb.push(Span::raw(" 󰝰 ").fg(THEME.crumb_icon).bg(color_a));
                format!("{name} ")
            };

            breadcrumb.push(Span::raw(name).fg(THEME.text).bg(color_a));

            if i < last_index {
                breadcrumb.push(Span::raw("").fg(color_a).bg(color_b));
            } else {
                breadcrumb.push(Span::raw("").fg(color_a).bg(THEME.background));
            }
        }

        (title, breadcrumb)
    } else {
        (
            Line::from("[Editor]".fg(THEME.inactive_text)),
            vec![Span::default()],
        )
    };

    let (mode, bg) = match context.notebook.state {
        ContextState::EditorNormalMode { .. } => (
            Span::raw(" NORMAL ").fg(THEME.text).bg(THEME.background),
            THEME.background,
        ),
        ContextState::EditorInsertMode => (
            Span::raw(" INSERT ").fg(THEME.accent_text).bg(THEME.accent),
            THEME.accent,
        ),
        ContextState::EditorVisualMode => (
            Span::raw(" VISUAL ").fg(THEME.error_text).bg(THEME.error),
            THEME.error,
        ),
        _ => (Span::raw("        ").bg(THEME.surface), THEME.surface),
    };

    if context.notebook.tab_index.is_some() {
        bottom_left.insert(0, mode);
        bottom_left.insert(1, Span::raw("").fg(bg).bg(THEME.crumb_a));
    }

    let bottom_left = Line::from(bottom_left);
    let block = Block::new().title(title).title_bottom(bottom_left);
    match (
        context.last_log.as_ref(),
        context.notebook.editors.iter().any(|(_, item)| item.dirty),
    ) {
        (_, true) => block.title_bottom(
            Line::from(vec![
                Span::raw("").fg(THEME.accent).bg(THEME.background),
                Span::raw(" 󰔚 Saving... ")
                    .fg(THEME.accent_text)
                    .bg(THEME.accent),
            ])
            .right_aligned(),
        ),
        (Some((log, _)), false) => block.title_bottom(
            Line::from(format!(" {log} ").fg(THEME.success_text).bg(THEME.success)).right_aligned(),
        ),
        (None, false) => block,
    }
    .fg(THEME.text)
    .bg(THEME.background)
    .padding(Padding::left(1))
}

/// Positions edtui's internal viewport for a pending scroll request (zt/zz/zb).
///
/// Because edtui doesn't expose viewport controls, we use two "pre-renders"
/// to a scratch buffer to nudge the viewport into the desired position.
/// Returns the number of rows to shift the buffer content up after the real
/// render, to handle the case where the desired viewport extends past the
/// end of the document.
///
/// The scroll anchor persists across frames so that j/k movements after a
/// scroll command do not cause the viewport to snap back.
fn prepare_scroll_viewport(context: &mut Context, area: Rect) -> usize {
    // Case A: new scroll command
    if let Some(scroll) = context.notebook.pending_scroll.take() {
        let editor = context.notebook.get_editor_mut();
        let cursor_row = editor.cursor.row;
        let visible_height = (area.height as usize).saturating_sub(2);
        let total_lines = editor.lines.len();

        let desired_top = match scroll {
            ScrollRequest::Top => cursor_row,
            ScrollRequest::Center => cursor_row.saturating_sub(visible_height / 2),
            ScrollRequest::Bottom => cursor_row.saturating_sub(visible_height.saturating_sub(1)),
        };

        let desired_bottom = desired_top + visible_height.saturating_sub(1);
        let clamped_bottom = desired_bottom.min(total_lines.saturating_sub(1));
        let actual_viewport_start = clamped_bottom.saturating_sub(visible_height.saturating_sub(1));
        let shift = desired_top.saturating_sub(actual_viewport_start);

        let real_cursor = editor.cursor;
        let scratch_theme = || {
            EditorTheme::default()
                .block(Block::bordered())
                .hide_status_line()
        };

        // Pre-render 1: reset viewport to top
        editor.cursor = Index2::new(0, 0);
        let mut scratch = Buffer::empty(area);
        EditorView::new(editor)
            .theme(scratch_theme())
            .wrap(true)
            .render(area, &mut scratch);

        // Pre-render 2: scroll viewport to best possible position
        let editor = context.notebook.get_editor_mut();
        editor.cursor = Index2::new(clamped_bottom, 0);
        let mut scratch = Buffer::empty(area);
        EditorView::new(editor)
            .theme(scratch_theme())
            .wrap(true)
            .render(area, &mut scratch);

        // Restore cursor
        let editor = context.notebook.get_editor_mut();
        editor.cursor = real_cursor;

        // Save anchor only when shift > 0 (document too short for full scroll)
        context.notebook.scroll_anchor = if shift > 0 {
            Some(ScrollAnchor {
                desired_top,
                actual_viewport_y: actual_viewport_start,
            })
        } else {
            None
        };

        return shift;
    }

    // Case B: persistent anchor from a previous scroll command
    if let Some(anchor) = context.notebook.scroll_anchor {
        let editor = context.notebook.get_editor_mut();
        let cursor_row = editor.cursor.row;

        // Clamp desired_top so cursor stays visible (when moving up with k)
        let effective_top = anchor.desired_top.min(cursor_row);
        let shift = effective_top.saturating_sub(anchor.actual_viewport_y);

        if shift == 0 {
            context.notebook.scroll_anchor = None;
        } else {
            context.notebook.scroll_anchor = Some(ScrollAnchor {
                desired_top: effective_top,
                actual_viewport_y: anchor.actual_viewport_y,
            });
        }

        return shift;
    }

    // Case C: no scroll state
    0
}

/// Shifts rendered content rows up to simulate scrolling past the document end.
fn apply_scroll_shift(buf: &mut Buffer, area: Rect, shift: usize) {
    let inner_y = (area.y + 1) as usize;
    let inner_height = (area.height as usize).saturating_sub(2);

    // Shift content rows up
    for row in 0..inner_height.saturating_sub(shift) {
        for col in 0..area.width as usize {
            let x = area.x + col as u16;
            let src_y = (inner_y + row + shift) as u16;
            let dst_y = (inner_y + row) as u16;
            let cell = buf[(x, src_y)].clone();
            buf[(x, dst_y)] = cell;
        }
    }

    // Fill vacated bottom rows with background
    let bg_style = Style::default().fg(THEME.text).bg(THEME.background);
    for row in inner_height.saturating_sub(shift)..inner_height {
        for col in 0..area.width as usize {
            let x = area.x + col as u16;
            let y = (inner_y + row) as u16;
            let cell = &mut buf[(x, y)];
            cell.reset();
            cell.set_style(bg_style);
        }
    }
}
