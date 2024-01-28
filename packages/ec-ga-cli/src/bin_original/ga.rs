use anyhow::Result;
use clap::Parser;
use ga::args::Args;
use ga::do_main;

fn main() -> Result<()> {
    let args = Args::parse();

    do_main(args)
}
