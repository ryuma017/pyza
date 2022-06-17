use anyhow::Context;
use pyza::config::get_configuration;

fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().context("Failed to read configuration file.")?;
    println!("{}", configuration.template);

    Ok(())
}
