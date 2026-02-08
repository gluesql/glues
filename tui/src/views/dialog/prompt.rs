use {
    crate::{
        context::{Context, ContextPrompt},
        logger::*,
        theme::THEME,
    },
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout, Position},
        style::{Style, Stylize},
        text::Line,
        widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap},
    },
};

pub fn draw(frame: &mut Frame, context: &mut Context) {
    let ContextPrompt {
        message,
        input,
        mask,
        ..
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
        .bg(THEME.surface)
        .fg(THEME.text)
        .padding(Padding::new(2, 2, 1, 1))
        .title(Line::from("Prompt").centered());
    let inner_area = block.inner(area);
    let [message_area, input_area, _, control_area] =
        Layout::vertical([Length(num_lines), Length(3), Length(1), Length(2)]).areas(inner_area);
    let message = Paragraph::new(message.clone())
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    let lines = vec![
        "[Enter] Submit".fg(THEME.text_secondary).into(),
        "[Esc] Cancel".fg(THEME.text_secondary).into(),
    ];
    let control = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    let input_block = Block::default()
        .border_style(Style::default())
        .borders(Borders::ALL);
    let input_inner = input_block.inner(input_area);
    let width = input_inner.width.max(1) as usize;
    let scroll = input.visual_scroll(width);

    let display_text = if let Some(mask_char) = mask {
        mask_char.to_string().repeat(input.value().len())
    } else {
        input.value().to_string()
    };
    let input_widget = Paragraph::new(display_text.as_str())
        .scroll((0, scroll as u16))
        .block(input_block);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(message, message_area);
    frame.render_widget(input_widget, input_area);
    frame.render_widget(control, control_area);

    let cursor_x = input_inner.x + (input.visual_cursor().max(scroll) - scroll) as u16;
    let cursor_y = input_inner.y;
    frame.set_cursor_position(Position::new(cursor_x, cursor_y));
}
