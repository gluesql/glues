use {
    crate::{context::Context, logger::*},
    ratatui::{
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::Style,
        widgets::{Block, Clear, Padding, Paragraph, Wrap},
        Frame,
    },
};

pub fn draw(frame: &mut Frame, context: &mut Context) {
    let [area] = Layout::horizontal([Length(40)])
        .flex(Flex::Center)
        .areas(frame.area());

    let [area] = Layout::vertical([Length(8)]).flex(Flex::Center).areas(area);

    let block = Block::bordered()
        .padding(Padding::new(2, 2, 1, 1))
        .title("Confirm")
        .title_alignment(Alignment::Center);

    let inner_area = block.inner(area);
    let (message, _) = context.confirm.as_ref().log_expect("fasdf");
    let message = format!("{message}\n\n[Y] Confirm\n[N] Cancel");
    let paragraph = Paragraph::new(message.as_str())
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, inner_area);
}
