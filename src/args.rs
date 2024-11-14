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
    /// Encode a message into a chunk
    Encode {
        /// Image path to encode message into
        path: PathBuf,
        /// Chunk type 4 letters alphabetical case sensitive to embed message into
        chunk_type: String,
        /// The message to embed
        message: String,
        /// Optinal output file to use instead of overwriting original
        output_file: Option<PathBuf>,
    },
    /// Decode an image to read an embeded message from it
    Decode {
        /// Image path to read message from
        path: PathBuf,
        /// Chunk type 4 letters alphabetical case sensitive to embed message into
        chunk_type: String,
    },
    /// Remove a chunk that contains a message
    Remove {
        /// Image path to remove chunk from
        path: PathBuf,
        /// Chunk type 4 letters alphabetical case sensitive to embed message into
        chunk_type: String,
    },
}
