mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;
use args::{Args, OperationMode};
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        OperationMode::Encode {
            path,
            chunk_type,
            message,
            output_file,
        } => {
        }
        OperationMode::Decode { path, chunk_type } => {}
        OperationMode::Remove { path, chunk_type } => {}
        OperationMode::Print { path } => {}
    }
    Ok(())
}
