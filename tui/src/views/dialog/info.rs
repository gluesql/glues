use {
    crate::{context::Context, logger::*, theme::THEME},
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::Stylize,
        text::Line,
        widgets::{Block, Clear, Padding, Paragraph, Wrap},
    },
};

pub fn draw(frame: &mut Frame, context: &mut Context) {
    let info = context
        .info
        .as_ref()
        .log_expect("info dialog data must exist");

    let width = 60;
    let content_height = info.lines.len().max(1) as u16;
    let height = (content_height + 6).min(frame.area().height);

    let [area] = Layout::horizontal([Length(width.min(frame.area().width))])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(height)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::bordered()
        .fg(THEME.text)
        .bg(THEME.surface)
        .padding(Padding::new(2, 2, 1, 1))
        .title(info.title.as_str())
        .title_alignment(Alignment::Center);

    let inner = block.inner(area);
    let [message_area, control_area] = Layout::vertical([Length(content_height + 1), Length(1)])
        .flex(Flex::SpaceBetween)
        .areas(inner);

    let mut lines = info.lines.clone();
    lines.push(Line::default());

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Left)
        .fg(THEME.text);
    let control = Line::from("Press Esc to close".fg(THEME.text_secondary)).centered();

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, message_area);
    frame.render_widget(control, control_area);
}
