use {
    crate::{context::Context, logger::*, theme::THEME},
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout, Rect},
        style::{Style, Stylize},
        widgets::{Block, Clear, HighlightSpacing, List, ListItem, Padding, Paragraph},
    },
};

pub fn draw(frame: &mut Frame, context: &mut Context) {
    let selector = context
        .theme_selector
        .as_mut()
        .log_expect("theme selector not found");

    let area = centered_area(frame.area());
    let block = Block::bordered()
        .bg(THEME.surface)
        .fg(THEME.text)
        .padding(Padding::new(2, 2, 1, 1))
        .title("Select Theme")
        .title_alignment(Alignment::Center);
    let inner = block.inner(area);

    let presets = selector.presets();
    let preset_items: Vec<ListItem> = presets
        .iter()
        .map(|preset| {
            let label = format!("[{}] {}", preset.key, preset.label);
            ListItem::new(label.fg(THEME.menu))
        })
        .collect();
    let list_height = presets.len() as u16;

    let [list_area, _, control_area] =
        Layout::vertical([Length(list_height + 1), Length(1), Length(1)]).areas(inner);

    let list = List::new(preset_items)
        .block(Block::default())
        .highlight_style(Style::new().fg(THEME.accent_text).bg(THEME.accent))
        .highlight_symbol(" ")
        .highlight_spacing(HighlightSpacing::Always);

    let control = Paragraph::new("[Enter] Apply  [Esc] Cancel".fg(THEME.inactive_text))
        .alignment(Alignment::Center);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_stateful_widget(list, list_area, &mut selector.list_state);
    frame.render_widget(control, control_area);
}

fn centered_area(area: Rect) -> Rect {
    let [area] = Layout::horizontal([Length(36)])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([Length(12)])
        .flex(Flex::Center)
        .areas(area);
    area
}
