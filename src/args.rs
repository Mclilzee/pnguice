use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {

    #[command(subcommand)]
    command: OperationMode,
}


#[derive(Debug, Subcommand)]
enum OperationMode {
    #[command(arg_required_else_help = true)]
    Encode {
        path: PathBuf,
    },
    Decode,
    Print,
    Remove,
}

#[derive(Parser, Debug)]
struct EncodeArgs {
}
