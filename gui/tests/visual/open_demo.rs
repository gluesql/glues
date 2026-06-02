use crate::support;

pub fn run() -> anyhow::Result<()> {
    support::capture_root("open-demo", 1200, 760, |window, cx| {
        glues_gui::app::build_visual_demo_root(window, cx)
    })?;
    Ok(())
}
