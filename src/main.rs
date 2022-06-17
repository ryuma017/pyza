use pyza::parser;

fn main() -> anyhow::Result<()> {
    parser::run()?;
    Ok(())
}
