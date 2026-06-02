use crate::support;

pub fn run() -> anyhow::Result<()> {
    support::capture_root("open-screen-recents", 900, 760, |window, cx| {
        glues_gui::app::build_visual_open_screen_root(window, cx)
    })?;
    Ok(())
}
