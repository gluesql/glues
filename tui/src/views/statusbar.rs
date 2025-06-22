use {
    crate::{
        context::{NotebookContext, notebook::ContextState},
        logger::*,
        theme,
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
    let t = theme::current_theme();
    let description = format!(" {}", state.describe().log_unwrap());
    let insert_mode = matches!(context.state, ContextState::EditorInsertMode);
    let [desc_area, keymap_area] =
        Layout::horizontal([Percentage(100), Length(if insert_mode { 23 } else { 18 })])
            .areas(area);

    frame.render_widget(
        Text::raw(description).fg(t.inactive_text).bg(t.panel),
        desc_area,
    );

    frame.render_widget(
        Line::from(vec![
            Span::raw("").fg(t.success).bg(t.panel),
            Span::raw(if insert_mode {
                " [Ctrl+h] Show keymap "
            } else {
                " [?] Show keymap "
            })
            .fg(t.success_text)
            .bg(t.success),
        ]),
        keymap_area,
    );
}
