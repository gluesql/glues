use {
    crate::{Context, theme::THEME},
    glues_core::types::KeymapGroup,
    ratatui::{
        Frame,
        layout::{
            Alignment,
            Constraint::{Length, Percentage},
            Flex, Layout,
        },
        style::{Style, Stylize},
        text::{Line, Span},
        widgets::{
            Block, Clear, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        },
    },
    textwrap::wrap,
};

const KEYMAP_WIDTH: u16 = 46;
const KEY_WIDTH: u16 = 10;

pub fn draw(frame: &mut Frame, context: &mut Context, keymap: &[KeymapGroup]) {
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

    let mut lines: Vec<Line> = Vec::new();
    for group in keymap {
        lines.push(
            Line::from(group.title.clone())
                .fg(THEME.warning_text)
                .bg(THEME.warning),
        );
        for item in &group.items {
            let wrapped = wrap(&item.desc, desc_width as usize);
            for (i, d) in wrapped.into_iter().enumerate() {
                if i == 0 {
                    lines.push(Line::from(vec![
                        Span::raw(format!("[{}] ", item.key)),
                        Span::raw(d.into_owned()),
                    ]));
                } else {
                    lines.push(Line::from(vec![
                        Span::raw(" ".repeat(KEY_WIDTH as usize)),
                        Span::raw(d.into_owned()),
                    ]));
                }
            }
        }
    }

    let total_lines = lines.len();
    let max_scroll = total_lines.saturating_sub(inner_area.height as usize);
    context.keymap_scroll = context.keymap_scroll.min(max_scroll);
    context.keymap_scroll_state = ScrollbarState::new(total_lines)
        .position(context.keymap_scroll)
        .viewport_content_length(inner_area.height as usize);

    frame.render_widget(Clear, area);
    frame.render_widget(block.clone(), area);

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Left)
        .style(Style::default())
        .scroll((context.keymap_scroll as u16, 0));

    frame.render_widget(paragraph, inner_area);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight),
        inner_area,
        &mut context.keymap_scroll_state,
    );
}
