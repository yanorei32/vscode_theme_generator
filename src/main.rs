use clap::Parser;
use cli::Cli;

mod cli;
mod determinator;
mod io;
mod model;
mod optimize;
mod util;
mod paths;

fn main() -> anyhow::Result<()> {
    Cli::parse().run()
}
