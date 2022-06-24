use anyhow::Result;
use changelog::Cli;
use clap::Parser;

fn main() -> Result<()> {
    Cli::parse().run()
}
