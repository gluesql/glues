use crate::support;

pub fn run() -> anyhow::Result<()> {
    support::capture_root("empty-viewer", 1200, 760, |window, cx| {
        glues_gui::app::build_visual_empty_viewer_root(window, cx)
    })?;
    Ok(())
}
