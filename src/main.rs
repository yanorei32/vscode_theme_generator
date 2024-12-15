use clap::Parser;
use cli::Cli;

mod cli;
mod optimize;
mod io;
mod model;
mod util;

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
