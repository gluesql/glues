use ratatui::{Terminal, backend::TestBackend};

pub trait TerminalTestExt {
    fn assert_contains(&self, needle: &str);
    fn assert_not_contains(&self, needle: &str);
    fn assert_snapshot(&self, name: &str);
}

impl TerminalTestExt for Terminal<TestBackend> {
    fn assert_contains(&self, needle: &str) {
        let text = buffer_to_lines_internal(self).join("\n");
        assert!(text.contains(needle));
    }

    fn assert_not_contains(&self, needle: &str) {
        let text = buffer_to_lines_internal(self).join("\n");
        assert!(!text.contains(needle));
    }

    fn assert_snapshot(&self, name: &str) {
        let lines = buffer_to_lines_internal(self);
        super::assert_snapshot(name, &lines);
    }
}

fn buffer_to_lines_internal(term: &Terminal<TestBackend>) -> Vec<String> {
    let buf = term.backend().buffer().clone();
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
