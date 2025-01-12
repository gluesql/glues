use {
    crate::context::{notebook::ContextState, Context},
    ratatui::{
        layout::Rect,
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Padding},
        Frame,
    },
    throbber_widgets_tui::Throbber,
    tui_textarea::TextArea,
};

pub fn draw(frame: &mut Frame, area: Rect, context: &mut Context) {
    context.notebook.editor_height = area.height - 2;

    let (title, breadcrumb) = if let Some(tab_index) = context.notebook.tab_index {
        let mut title = vec!["[".into()];
        for (i, tab) in context.notebook.tabs.iter().enumerate() {
            let name = tab.note.name.clone();
            let name = if i == tab_index {
                if context.notebook.state.is_editor() {
                    name.white().on_blue()
                } else {
                    name.white().on_dark_gray()
                }
            } else {
                name.dark_gray()
            };

            if i != 0 {
                title.push("|".into());
            }
            title.push(name);
        }
        title.push("]".into());

        let title = Line::from(title);
        let breadcrumb = Span::raw(format!(
            " {} ",
            context.notebook.tabs[tab_index].breadcrumb.join("/")
        ))
        .black()
        .on_green();
        (title, breadcrumb)
    } else {
        (Line::from("[Editor]".dark_gray()), Span::default())
    };

    let mode = match context.notebook.state {
        ContextState::EditorNormalMode { .. } => Span::raw(" NORMAL ").white().on_black(),
        ContextState::EditorInsertMode => Span::raw(" INSERT ").black().on_yellow(),
        ContextState::EditorVisualMode => Span::raw(" VISUAL ").white().on_red(),
        _ => Span::raw("        ").on_dark_gray(),
    };

    let bottom_left = Line::from(vec![mode, breadcrumb]);
    let block = Block::new().title(title).title_bottom(bottom_left);
    let block = match (
        context.last_log.as_ref(),
        context.notebook.tabs.iter().any(|tab| tab.dirty),
    ) {
        (_, true) => {
            let throbber = Throbber::default()
                .label("Saving...")
                .style(Style::default().yellow());
            block.title_bottom(Line::from(throbber).right_aligned())
        }
        (Some((log, _)), false) => {
            block.title_bottom(Line::from(log.clone().green()).right_aligned())
        }
        (None, false) => block,
    }
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
            Style::default().white().on_blue(),
            Style::default().underlined(),
        ),
        _ => (Style::default(), Style::default()),
    };

    editor.set_cursor_style(cursor_style);
    editor.set_cursor_line_style(cursor_line_style);
    if show_line_number {
        editor.set_line_number_style(Style::default().dark_gray().dim());
    } else {
        editor.remove_line_number();
    }

    frame.render_widget(&*editor, area);
}
