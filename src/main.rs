use clap::Parser;
use cli::Cli;

mod cli;
mod optimize;
mod palette;
mod setting;
mod model;
mod util;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()?;
    Ok(())
}
