#[cfg(target_os = "macos")]
use anyhow::Context as _;

#[cfg(target_os = "macos")]
#[path = "visual/empty_viewer.rs"]
mod empty_viewer;

#[cfg(target_os = "macos")]
#[path = "visual/long_note.rs"]
mod long_note;

#[cfg(target_os = "macos")]
#[path = "visual/open_demo.rs"]
mod open_demo;

#[cfg(target_os = "macos")]
#[path = "visual/open_screen_recents.rs"]
mod open_screen_recents;

#[cfg(target_os = "macos")]
#[path = "visual/workspace_narrow.rs"]
mod workspace_narrow;

#[cfg(target_os = "macos")]
#[path = "visual/support.rs"]
mod support;

#[cfg(target_os = "macos")]
type VisualCase = (&'static str, fn() -> anyhow::Result<()>);

#[cfg(target_os = "macos")]
fn main() {
    if let Err(error) = run() {
        eprintln!("{error:?}");
        std::process::exit(1);
    }
}

#[cfg(target_os = "macos")]
fn run() -> anyhow::Result<()> {
    let cases: &[VisualCase] = &[
        ("open-screen-recents", open_screen_recents::run),
        ("open-demo", open_demo::run),
        ("empty-viewer", empty_viewer::run),
        ("long-note", long_note::run),
        ("workspace-narrow", workspace_narrow::run),
    ];

    for (name, run) in cases {
        println!("running {name}");
        run().with_context(|| format!("visual case failed: {name}"))?;
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("glues-gui visual screenshots are currently macOS-only");
}
