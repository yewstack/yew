use anyhow::Result;
use changelog::Cli;
use structopt::StructOpt;

fn main() -> Result<()> {
    Cli::from_args().run()
}
