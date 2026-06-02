use crate::support;

pub fn run() -> anyhow::Result<()> {
    support::capture_root("workspace-narrow", 760, 620, |window, cx| {
        glues_gui::app::build_visual_demo_root(window, cx)
    })?;
    Ok(())
}
