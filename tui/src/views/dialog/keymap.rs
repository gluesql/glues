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
        widgets::{Block, Clear, Padding, Paragraph},
    },
    textwrap::wrap,
};

const KEYMAP_WIDTH: u16 = 46;
const KEY_WIDTH: u16 = 10;

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
    let desc_width = inner_area.width.saturating_sub(KEY_WIDTH + 1);

    let heights: Vec<u16> = keymap
        .iter()
        .map(|item| wrap(&item.desc, desc_width as usize).len() as u16)
        .collect();

    let row_constraints = heights.iter().map(|h| Length(*h)).collect::<Vec<_>>();
    let rows = Layout::vertical(row_constraints).split(inner_area);

    frame.render_widget(Clear, area);
    frame.render_widget(block.clone(), area);

    for (row_area, item) in rows.iter().zip(keymap) {
        let [key_area, desc_area] =
            Layout::horizontal([Length(KEY_WIDTH), Length(desc_width)]).areas(*row_area);

        let key_paragraph = Paragraph::new(Line::from(vec![Span::raw(format!("[{}]", item.key))]))
            .alignment(Alignment::Left);
        let desc_lines: Vec<Line> = wrap(&item.desc, desc_width as usize)
            .into_iter()
            .map(|c| Line::from(c.into_owned()))
            .collect();
        let desc_paragraph = Paragraph::new(desc_lines)
            .alignment(Alignment::Left)
            .style(Style::default());

        frame.render_widget(key_paragraph, key_area);
        frame.render_widget(desc_paragraph, desc_area);
    }
}
