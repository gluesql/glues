use {
    crate::theme::THEME,
    glues_core::types::KeymapItem,
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

const KEYMAP_WIDTH: u16 = 60;
const KEY_WIDTH: usize = 12;

pub fn draw(frame: &mut Frame, keymap: &[KeymapItem]) {
    let [area] = Layout::horizontal([Length(KEYMAP_WIDTH)])
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
    let message: Vec<Line> = keymap
        .iter()
        .map(|v| format!("[{:<key_width$}] {}", v.key, v.desc, key_width = KEY_WIDTH).into())
        .collect();
    let paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .style(Style::default())
        .alignment(Alignment::Left);

    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    frame.render_widget(paragraph, inner_area);
}
