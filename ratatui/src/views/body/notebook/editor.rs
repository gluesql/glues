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
    .padding(Padding::horizontal(1));

    context.notebook.editor.set_block(block);

    let (cursor_style, cursor_line_style) = match context.notebook.state {
        ContextState::EditorEditMode => (
            Style::default().white().on_blue(),
            Style::default().underlined(),
        ),
        _ => (Style::default(), Style::default()),
    };

    context.notebook.editor.set_cursor_style(cursor_style);
    context
        .notebook
        .editor
        .set_cursor_line_style(cursor_line_style);

    frame.render_widget(&context.notebook.editor, area);
}
