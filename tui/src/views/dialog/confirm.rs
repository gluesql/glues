use {
    crate::{context::Context, logger::*, theme::THEME},
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Style, Stylize},
        widgets::{Block, Clear, Padding, Paragraph, Wrap},
    },
};

pub fn draw(frame: &mut Frame, context: &mut Context) {
    let [area] = Layout::horizontal([Length(40)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(9)]).flex(Flex::Center).areas(area);

    let block = Block::bordered()
        .bg(THEME.surface)
        .fg(THEME.text)
        .padding(Padding::new(2, 2, 1, 1))
        .title("Confirm")
        .title_alignment(Alignment::Center);
    let inner_area = block.inner(area);
    let [message_area, control_area] = Layout::vertical([Length(4), Length(2)])
        .flex(Flex::SpaceBetween)
        .areas(inner_area);

    let (message, _) = context
        .confirm
        .as_ref()
        .log_expect("confirm message not found");
    let message = Paragraph::new(message.as_str())
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    let lines = vec![
        "[y] Confirm".fg(THEME.text_secondary).into(),
        "[n] Cancel".fg(THEME.text_secondary).into(),
    ];
    let control = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(message, message_area);
    frame.render_widget(control, control_area);
}
