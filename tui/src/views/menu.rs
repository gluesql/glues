use {
    crate::{
        context::menu::{MENU_ITEMS, MenuContext},
        theme::THEME,
    },
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Style, Stylize},
        widgets::{Block, Clear, HighlightSpacing, List, ListDirection, Padding},
    },
};

pub fn draw(frame: &mut Frame, context: &mut MenuContext) {
    let [area] = Layout::horizontal([Length(20)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(7)]).flex(Flex::Center).areas(area);

    let block = Block::bordered()
        .bg(THEME.surface)
        .fg(THEME.text)
        .padding(Padding::new(2, 2, 1, 1))
        .title("Menu")
        .title_alignment(Alignment::Center);
    let list = List::new(MENU_ITEMS)
        .block(block)
        .highlight_style(Style::new().fg(THEME.accent_text).bg(THEME.accent))
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut context.list_state);
}
