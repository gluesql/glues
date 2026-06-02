use {
    anyhow::{Context as _, ensure},
    gpui::{App, Entity, HeadlessAppContext, Platform, Window, px, size},
    gpui_component::Root,
    gpui_macos::metal_renderer::MetalHeadlessRenderer,
    std::{fs, path::PathBuf, rc::Rc, sync::Arc},
};

pub fn capture_root(
    name: &str,
    width: u32,
    height: u32,
    build_root: impl FnOnce(&mut Window, &mut App) -> Entity<Root>,
) -> anyhow::Result<PathBuf> {
    let platform = Rc::new(gpui_macos::MacPlatform::new(true));
    let mut cx = HeadlessAppContext::with_platform(
        platform.text_system(),
        Arc::new(gpui_component_assets::Assets),
        || Some(Box::new(MetalHeadlessRenderer::new())),
    );

    cx.update(glues_gui::app::init);

    let window = cx.open_window(size(px(width as f32), px(height as f32)), build_root)?;
    cx.update(|cx| cx.refresh_windows());
    cx.run_until_parked();

    let screenshot = cx.capture_screenshot(window.into())?;
    let scale_x = screenshot.width() / width;
    let scale_y = screenshot.height() / height;
    ensure!(
        scale_x >= 1
            && scale_x == scale_y
            && screenshot.width() == width * scale_x
            && screenshot.height() == height * scale_y,
        "unexpected screenshot size: {}x{}",
        screenshot.width(),
        screenshot.height()
    );

    let first_pixel = *screenshot.get_pixel(0, 0);
    ensure!(
        screenshot.pixels().any(|pixel| *pixel != first_pixel),
        "screenshot is a uniform image"
    );

    let output_path = visual_output_path(name)?;
    fs::create_dir_all(
        output_path
            .parent()
            .context("visual output path has no parent")?,
    )?;
    screenshot.save(&output_path)?;
    println!("wrote {}", output_path.display());

    Ok(output_path)
}

fn visual_output_path(name: &str) -> anyhow::Result<PathBuf> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("gui crate has no workspace parent")?
        .to_path_buf();
    Ok(workspace_root
        .join("target")
        .join("visual-tests")
        .join("glues-gui")
        .join(format!("{name}.png")))
}
