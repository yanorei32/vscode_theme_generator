use clap::Parser;
use cli::Cli;

mod cli;
mod color;
mod optimize;
mod palette;
mod setting;
mod model;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()?;
    Ok(())
}
