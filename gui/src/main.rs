use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    glues_gui::run();
    Ok(())
}
