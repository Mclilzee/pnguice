use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: OperationMode,
}

#[derive(Debug, Subcommand)]
pub enum OperationMode {
    Encode {
        path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    Decode {
        path: PathBuf,
        chunk_type: String,
    },
    Remove {
        path: PathBuf,
        chunk_type: String,
    },
    Print {
        path: PathBuf,
    },
}
