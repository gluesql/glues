use {
    crate::{color::*, context::Context, logger::*},
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Style, Stylize},
        text::Line,
        widgets::{Block, Clear, Padding, Paragraph, Wrap},
    },
};

pub fn draw(frame: &mut Frame, context: &mut Context) {
    let [area] = Layout::horizontal([Length(45)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(8)]).flex(Flex::Center).areas(area);

    let block = Block::bordered()
        .fg(WHITE)
        .bg(GRAY_DARK)
        .padding(Padding::new(2, 2, 1, 1))
        .title("Alert")
        .title_alignment(Alignment::Center);

    let inner_area = block.inner(area);
    let [message_area, control_area] = Layout::vertical([Length(3), Length(1)])
        .flex(Flex::SpaceBetween)
        .areas(inner_area);

    let message: Line = context
        .alert
        .as_ref()
        .log_expect("alert message not found")
        .as_str()
        .into();
    let paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);
    let control = Line::from("Press any key to close".fg(GRAY_LIGHT)).centered();

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, message_area);
    frame.render_widget(control, control_area);
}
