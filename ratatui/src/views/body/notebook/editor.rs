use {
    crate::context::{notebook::ContextState, NotebookContext},
    edtui::{EditorTheme, EditorView},
    ratatui::{
        layout::Rect,
        style::{Style, Stylize},
        widgets::{Block, Padding},
        Frame,
    },
};

pub fn draw(frame: &mut Frame, area: Rect, context: &mut NotebookContext) {
    let (title, cursor_style) = match context.opened_note {
        Some(ref note) => (
            format!("[Editor: {}]", note.name),
            Style::default().white().on_blue(),
        ),
        None => ("[Editor]".to_string(), Style::default()),
    };
    let title = if matches!(
        context.state,
        ContextState::EditorViewMode | ContextState::EditorEditMode
    ) {
        title.blue()
    } else {
        title.dark_gray()
    };
    let block = Block::bordered()
        .title(title)
        .padding(Padding::horizontal(1));

    let theme = EditorTheme::default()
        .block(block)
        .base(Style::default())
        .cursor_style(cursor_style)
        .selection_style(Style::default())
        .hide_status_line();

    let editor = EditorView::new(&mut context.editor_state).theme(theme);

    frame.render_widget(editor, area);
}
