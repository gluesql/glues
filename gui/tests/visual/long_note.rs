use crate::support;

pub fn run() -> anyhow::Result<()> {
    support::capture_root("long-note", 900, 640, |window, cx| {
        glues_gui::app::build_visual_long_note_root(window, cx)
    })?;
    Ok(())
}
