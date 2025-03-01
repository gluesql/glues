use {
    crate::{
        color::*,
        context::{Context, ContextPrompt},
        logger::*,
    },
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Style, Stylize},
        widgets::{Block, Clear, Padding, Paragraph, Wrap},
    },
};

pub fn draw(frame: &mut Frame, context: &mut Context) {
    let ContextPrompt {
        message, widget, ..
    } = context
        .prompt
        .as_ref()
        .log_expect("prompt message not found");
    let num_lines = message.len() as u16;

    let [area] = Layout::horizontal([Length(61)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(num_lines + 10)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::bordered()
        .bg(GRAY_DARK)
        .fg(WHITE)
        .padding(Padding::new(2, 2, 1, 1))
        .title("Prompt")
        .title_alignment(Alignment::Center);
    let inner_area = block.inner(area);
    let [message_area, input_area, _, control_area] =
        Layout::vertical([Length(num_lines), Length(3), Length(1), Length(2)]).areas(inner_area);
    let message = Paragraph::new(message.clone())
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    let lines = vec![
        "[Enter] Submit".fg(GRAY_LIGHT).into(),
        "[Esc] Cancel".fg(GRAY_LIGHT).into(),
    ];
    let control = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(message, message_area);
    frame.render_widget(widget, input_area);
    frame.render_widget(control, control_area);
}
