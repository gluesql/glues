use {
    crate::{
        context::{NotebookContext, notebook::DIRECTORY_ACTIONS},
        theme::THEME,
    },
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Style, Stylize},
        widgets::{Block, Clear, HighlightSpacing, List, ListDirection, Padding},
    },
};

pub fn draw(frame: &mut Frame, context: &mut NotebookContext) {
    let [area] = Layout::horizontal([Length(28)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(9)]).flex(Flex::Center).areas(area);

    let block = Block::bordered()
        .bg(THEME.surface)
        .fg(THEME.text)
        .padding(Padding::new(2, 2, 1, 1))
        .title("Directory Actions")
        .title_alignment(Alignment::Center);
    let list = List::new(DIRECTORY_ACTIONS)
        .block(block)
        .highlight_style(Style::new().fg(THEME.accent_text).bg(THEME.accent))
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut context.directory_actions_state);
}
