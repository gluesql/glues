use {
    crate::{context::EntryContext, context::entry::THEMES, theme},
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Style, Stylize},
        widgets::{Block, Clear, HighlightSpacing, List, ListDirection, Padding},
    },
};

pub fn draw(frame: &mut Frame, context: &mut EntryContext) {
    let t = theme::current_theme();
    let [area] = Layout::horizontal([Length(28)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(9)]).flex(Flex::Center).areas(area);

    let block = Block::bordered()
        .bg(t.surface)
        .fg(t.text)
        .padding(Padding::new(2, 2, 1, 1))
        .title("Theme")
        .title_alignment(Alignment::Center);
    let list = List::new(THEMES)
        .block(block)
        .highlight_style(Style::new().fg(t.accent_text).bg(t.accent))
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut context.theme_state);
}
