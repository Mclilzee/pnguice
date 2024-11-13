mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;
use clap::Parser;
use args::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    Ok(())
}
