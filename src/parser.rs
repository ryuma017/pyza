use clap::{Parser, Subcommand};

use crate::actions::{start, StartOpt};

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Pyza {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[clap(name = "start")]
    Start(StartOpt),
}

pub fn run() -> Result<(), anyhow::Error> {
    let pyza = Pyza::parse();
    match pyza.action {
        Action::Start(opt) => start(opt),
    }
}
