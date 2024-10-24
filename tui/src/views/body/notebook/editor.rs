use {
    crate::context::{notebook::ContextState, Context},
    ratatui::{
        layout::{Alignment, Rect},
        style::{Style, Stylize},
        widgets::{
            block::{Position, Title},
            Block, Padding,
        },
        Frame,
    },
};

pub fn draw(frame: &mut Frame, area: Rect, context: &mut Context) {
    let title = match context.notebook.opened_note {
        Some(ref note) => format!("[Editor: {}]", note.name),
        None => "[Editor]".to_string(),
    };
    let title = if matches!(
        context.notebook.state,
        ContextState::EditorViewMode | ContextState::EditorEditMode
    ) {
        title.light_blue()
    } else {
        title.dark_gray()
    };

    let block = Block::bordered().title(title);
    let block = match context.last_log.as_ref() {
        Some((log, _)) => block.title(
            Title::from(log.clone().green())
                .position(Position::Bottom)
                .alignment(Alignment::Right),
        ),
        None => block,
    }
    .padding(if context.notebook.show_line_number {
        Padding::ZERO
    } else {
        Padding::left(1)
    });

    context.notebook.editor.set_block(block);

    let (cursor_style, cursor_line_style) = match context.notebook.state {
        ContextState::EditorEditMode => (
            Style::default().white().on_blue(),
            Style::default().underlined(),
        ),
        _ => (Style::default(), Style::default()),
    };

    let editor = &mut context.notebook.editor;
    editor.set_cursor_style(cursor_style);
    editor.set_cursor_line_style(cursor_line_style);
    if context.notebook.show_line_number {
        editor.set_line_number_style(Style::default().dark_gray().dim());
    } else {
        editor.remove_line_number();
    }

    frame.render_widget(&context.notebook.editor, area);
}
