#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use anyhow::Result;
use clap::Parser;
use rust_ga::args::Args;
use rust_ga::do_main;

fn main() -> Result<()> {
    let args = Args::parse();

    do_main(args)
}
