use std::fs;
use std::path::Path;

use anyhow::Context as _;

use crate::config::get_configuration;

#[derive(clap::Parser)]
pub struct StartOpt {
    challenge_name: String,
}

pub fn start(opt: StartOpt) -> anyhow::Result<()> {
    let configuration = get_configuration().context("Failed to read configuration file.")?;
    let template = configuration.template;
    let python_file = Path::new(&opt.challenge_name).with_extension("py");
    fs::write(python_file, template).context("Failed to write file.")?;
    Ok(())
}
