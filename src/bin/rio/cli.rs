use clap::{Parser, Subcommand};
use color_eyre::Result;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
#[clap(bin_name = "cargo")]
pub enum Command {
    Deploy(crate::deploy::DeploymentArgs),
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Command::Deploy(x) => x.exec(),
        }
    }
}
