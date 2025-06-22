use {
    crate::theme,
    glues_core::types::{KeymapGroup, KeymapItem},
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

pub fn draw(frame: &mut Frame, keymap: &[KeymapGroup]) {
    let t = theme::current_theme();
    let [area] = Layout::horizontal([Length(KEYMAP_WIDTH)])
        .flex(Flex::End)
        .areas(frame.area());
    let [area] = Layout::vertical([Percentage(100)])
        .flex(Flex::Center)
        .areas(area);

    let block = Block::default()
        .fg(t.inactive_text)
        .bg(t.panel)
        .padding(Padding::new(2, 2, 1, 1))
        .title(
            Line::from(vec![
                Span::raw("").fg(t.success).bg(t.panel),
                Span::raw(" [?] Hide keymap ")
                    .fg(t.success_text)
                    .bg(t.success),
            ])
            .right_aligned(),
        );

    let inner_area = block.inner(area);
    let desc_width = inner_area.width.saturating_sub(KEY_WIDTH + 1);

    enum Row<'a> {
        Title(&'a str),
        Item(&'a KeymapItem),
    }

    let mut rows = Vec::new();
    for group in keymap {
        rows.push(Row::Title(&group.title));
        for item in &group.items {
            rows.push(Row::Item(item));
        }
    }

    let heights: Vec<u16> = rows
        .iter()
        .map(|row| match row {
            Row::Title(_) => 1,
            Row::Item(item) => wrap(&item.desc, desc_width as usize).len() as u16,
        })
        .collect();

    let row_constraints = heights.iter().map(|h| Length(*h)).collect::<Vec<_>>();
    let row_areas = Layout::vertical(row_constraints).split(inner_area);

    frame.render_widget(Clear, area);
    frame.render_widget(block.clone(), area);

    for (row_area, row) in row_areas.iter().zip(rows) {
        match row {
            Row::Title(title) => {
                let line = Line::from(title.to_string()).fg(t.accent_text).bg(t.accent);
                let paragraph = Paragraph::new(line).alignment(Alignment::Left);
                frame.render_widget(paragraph, *row_area);
            }
            Row::Item(item) => {
                let [key_area, desc_area] =
                    Layout::horizontal([Length(KEY_WIDTH), Length(desc_width)]).areas(*row_area);

                let key_paragraph =
                    Paragraph::new(Line::from(vec![Span::raw(format!("[{}]", item.key))]))
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
    }
}
