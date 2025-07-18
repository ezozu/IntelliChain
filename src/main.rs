// src/main.rs
/*
 * Main executable for IntelliChain
 */

use clap::Parser;
use intellichain::{Result, run};

#[derive(Parser)]
#[command(version, about = "IntelliChain - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
