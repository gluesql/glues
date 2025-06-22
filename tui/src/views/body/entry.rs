use {
    crate::{
        context::{EntryContext, entry::MENU_ITEMS},
        theme,
    },
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout, Rect},
        style::{Style, Stylize},
        widgets::{Block, HighlightSpacing, List, ListDirection, Padding},
    },
    tui_big_text::BigText,
};

pub fn draw(frame: &mut Frame, area: Rect, context: &mut EntryContext) {
    let t = theme::current_theme();
    let background = Block::default().bg(t.background);
    frame.render_widget(background, area);

    let [area] = Layout::horizontal([Length(38)])
        .flex(Flex::Center)
        .areas(area);
    let [title_area, area] = Layout::vertical([Length(9), Length(12)])
        .flex(Flex::Center)
        .areas(area);

    let title = BigText::builder()
        .lines(vec!["Glues".fg(t.accent).into()])
        .build();
    let block = Block::bordered()
        .fg(t.text)
        .padding(Padding::new(2, 2, 1, 1))
        .title("Open Notes")
        .title_alignment(Alignment::Center);

    let items = MENU_ITEMS.into_iter().map(|name| {
        if name.ends_with("CSV") || name.ends_with("JSON") {
            name.fg(t.inactive_text)
        } else {
            name.fg(t.menu)
        }
    });
    let list = List::new(items)
        .block(block)
        .highlight_style(Style::new().fg(t.accent_text).bg(t.accent))
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always)
        .direction(ListDirection::TopToBottom);

    frame.render_widget(title, title_area);
    frame.render_stateful_widget(list, area, &mut context.list_state);
}
