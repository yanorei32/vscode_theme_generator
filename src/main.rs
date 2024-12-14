use clap::Parser;
use cli::Cli;

mod cli;
mod optimize;
mod palette;
mod io;
mod model;
mod util;

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
