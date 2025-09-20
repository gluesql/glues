use {
    crate::{context::Context, logger::*, theme::THEME},
    ratatui::{
        Frame,
        layout::{Alignment, Constraint::Length, Flex, Layout},
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Clear, Padding, Paragraph, Wrap},
    },
};

pub fn draw(frame: &mut Frame, context: &mut Context) {
    let [area] = Layout::horizontal([Length(44)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Length(11)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::bordered()
        .bg(THEME.surface)
        .fg(THEME.text)
        .padding(Padding::new(2, 2, 1, 1))
        .title("Notebook")
        .title_alignment(Alignment::Center);
    let inner_area = block.inner(area);
    let [message_area, options_area] = Layout::vertical([Length(4), Length(4)])
        .flex(Flex::SpaceBetween)
        .areas(inner_area);

    let menu = context
        .quit_menu
        .as_ref()
        .log_expect("quit menu must be some");

    let message = Paragraph::new(menu.message.as_str())
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    let key_style = Style::default().fg(THEME.text);
    let hint_style = Style::default().fg(THEME.text_secondary);

    let mut lines = vec![Line::from(vec![
        Span::styled("[m]", key_style),
        Span::raw(" "),
        Span::styled("Back to menu", hint_style),
    ])];

    #[cfg(not(target_arch = "wasm32"))]
    {
        lines.push(Line::from(vec![
            Span::styled("[q]", key_style),
            Span::raw(" "),
            Span::styled("Quit", hint_style),
        ]));
    }

    lines.push(Line::from(vec![
        Span::styled("[Esc]", key_style),
        Span::raw(" "),
        Span::styled("Cancel", hint_style),
    ]));

    let options = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(message, message_area);
    frame.render_widget(options, options_area);
}
