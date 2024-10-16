use {
    crate::context::{notebook::ContextState, Context},
    edtui::{EditorTheme, EditorView},
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
    let (title, cursor_style) = match context.notebook.opened_note {
        Some(ref note) => (
            format!("[Editor: {}]", note.name),
            Style::default().white().on_blue(),
        ),
        None => ("[Editor]".to_string(), Style::default()),
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

    let theme = EditorTheme::default()
        .block(block)
        .base(Style::default())
        .cursor_style(cursor_style)
        .selection_style(Style::default())
        .hide_status_line();

    let editor = EditorView::new(&mut context.notebook.editor_state).theme(theme);

    frame.render_widget(editor, area);
}
