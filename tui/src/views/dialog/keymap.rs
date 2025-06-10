use {
    crate::theme::THEME,
    ratatui::{
        Frame,
        layout::{
            Alignment,
            Constraint::{Length, Percentage},
            Flex, Layout,
        },
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{Block, Clear, Padding, Paragraph, Wrap},
    },
};

pub fn draw(frame: &mut Frame, keymap: &[String]) {
    let width = keymap.iter().map(|s| s.len()).max().unwrap_or_default() as u16;
    let width = width.max(20);
    let [area] = Layout::horizontal([Length(width + 5)])
        .flex(Flex::End)
        .areas(frame.area());
    let [area] = Layout::vertical([Percentage(100)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::default()
        .fg(THEME.inactive_text)
        .bg(THEME.panel)
        .padding(Padding::new(2, 2, 1, 1))
        .title(
            Line::from(vec![
                Span::raw("").fg(THEME.success).bg(THEME.panel),
                Span::raw(" [?] Hide keymap ")
                    .fg(THEME.success_text)
                    .bg(THEME.success),
            ])
            .right_aligned(),
        );

    let inner_area = block.inner(area);
    let message: Vec<Line> = keymap.iter().map(|v| v.as_str().into()).collect();
    let paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, inner_area);
}
