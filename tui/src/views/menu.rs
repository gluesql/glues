use {
    crate::{context::menu::{MenuContext, MENU_ITEMS}, theme::THEME},
    ratatui::{
        Frame,
        layout::Rect,
        text::{Line, Span},
        style::Stylize,
    },
};

pub fn draw(frame: &mut Frame, area: Rect, context: &mut MenuContext) {
    let mut spans = Vec::new();
    for (i, item) in MENU_ITEMS.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw(" "));
        }
        let span = if Some(i) == context.list_state.selected() {
            Span::raw(*item).fg(THEME.accent_text).bg(THEME.accent)
        } else {
            Span::raw(*item).fg(THEME.menu).bg(THEME.panel)
        };
        spans.push(span);
    }
    frame.render_widget(Line::from(spans).bg(THEME.panel), area);
}
