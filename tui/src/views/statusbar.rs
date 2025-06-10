use {
    crate::{
        context::{NotebookContext, notebook::ContextState},
        logger::*,
        theme::THEME,
    },
    glues_core::state::State,
    ratatui::{
        Frame,
        layout::{
            Constraint::{Length, Percentage},
            Layout, Rect,
        },
        style::Stylize,
        text::{Line, Span, Text},
    },
};

pub fn draw(frame: &mut Frame, area: Rect, state: &State, context: &NotebookContext) {
    let description = format!(" {}", state.describe().log_unwrap());
    let insert_mode = matches!(context.state, ContextState::EditorInsertMode);
    let [desc_area, keymap_area] =
        Layout::horizontal([Percentage(100), Length(if insert_mode { 23 } else { 18 })])
            .areas(area);

    frame.render_widget(
        Text::raw(description).fg(THEME.dim).bg(THEME.neutral_white),
        desc_area,
    );

    frame.render_widget(
        Line::from(vec![
            Span::raw("").fg(THEME.success).bg(THEME.neutral_white),
            Span::raw(if insert_mode {
                " [Ctrl+h] Show keymap "
            } else {
                " [?] Show keymap "
            })
            .fg(THEME.success_text)
            .bg(THEME.success),
        ]),
        keymap_area,
    );
}
