use ratatui::{Terminal, backend::TestBackend};

pub trait TerminalTestExt {
    fn buffer_to_lines(&self) -> Vec<String>;
}

impl TerminalTestExt for Terminal<TestBackend> {
    fn buffer_to_lines(&self) -> Vec<String> {
        let buf = self.backend().buffer().clone();
        let area = buf.area();
        let mut lines = Vec::with_capacity(area.height as usize);
        for y in 0..area.height {
            let mut line = String::new();
            for x in 0..area.width {
                line.push_str(buf[(x, y)].symbol());
            }
            lines.push(line);
        }
        lines
    }
}
