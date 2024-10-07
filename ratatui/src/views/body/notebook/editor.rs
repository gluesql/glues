use {
    crate::context::NotebookContext,
    glues_core::state::NotebookState,
    ratatui::{
        layout::Rect,
        style::Stylize,
        text::Line,
        widgets::{Block, Paragraph},
        Frame,
    },
};

pub fn draw(frame: &mut Frame, area: Rect, _state: &NotebookState, _context: &mut NotebookContext) {
    let block = Block::bordered().title("Editor");
    let inner_area = block.inner(area);

    frame.render_widget(block, area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(" "),
            Line::from("Notebook"),
            Line::from("Horizontal Layout Example. Press q to quit".dark_gray()).centered(),
            Line::from("Each line has 2 constraints, plus Min(0) to fill the remaining space."),
            Line::from("E.g. the second line of the Len/Min box is [Length(2), Min(2), Min(0)]"),
            Line::from("Note: constraint labels that don't fit are truncated"),
        ]),
        inner_area,
    );
}
