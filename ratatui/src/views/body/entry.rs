use {
    crate::context::EntryContext,
    ratatui::{
        layout::{Alignment, Constraint::Length, Flex, Layout, Rect},
        style::{Color, Style, Stylize},
        widgets::{Block, HighlightSpacing, List, ListDirection, Padding},
        Frame,
    },
    tui_big_text::BigText,
};

pub fn draw(frame: &mut Frame, area: Rect, context: &mut EntryContext) {
    let [area] = Layout::horizontal([Length(38)])
        .flex(Flex::Center)
        .areas(area);
    let [title_area, area] = Layout::vertical([Length(9), Length(9)])
        .flex(Flex::Center)
        .areas(area);

    let title = BigText::builder()
        .lines(vec!["Glues".dark_gray().into()])
        .build();
    let block = Block::bordered()
        .padding(Padding::new(2, 2, 1, 1))
        .title("Open Notes")
        .title_alignment(Alignment::Center);

    let items = ["Instant", "CSV", "JSON", "File", "Git"];
    let list = List::new(items)
        .block(block)
        .highlight_style(Style::new().fg(Color::White).bg(Color::Blue))
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(title, title_area);
    frame.render_stateful_widget(list, area, &mut context.list_state);
}
