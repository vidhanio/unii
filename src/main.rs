fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    unii::run()?;

    Ok(())
}
