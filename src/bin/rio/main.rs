//! `cargo rio`

mod cli;
mod deploy;

use clap::Parser;
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

fn main() -> color_eyre::Result<()> {
    let cli = cli::Cli::parse();

    if cli.verbose {
        let filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("trace"))?;
        tracing_subscriber::fmt()
            .compact()
            .with_env_filter(filter)
            .finish()
            .init();
    }

    cli.exec()?;
    Ok(())
}
