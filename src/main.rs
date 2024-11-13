mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use anyhow::Result;
use args::{Args, OperationMode};
use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use png::Png;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        OperationMode::Encode {
            path,
            chunk_type,
            message,
            output_file,
        } => {
            let mut file = File::open(&path)?;
            let chunk_type: ChunkType = chunk_type.parse()?;
            let chunk = Chunk::new(chunk_type, message.bytes().collect());

            let mut reader = BufReader::new(&file);
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            let mut png = Png::try_from(buf.as_ref())?;
            png.append_chunk(chunk);

            if let Some(output) = output_file {
                file = File::create_new(output)?;
            }

            file.write_all(&png.as_bytes())?;
        }
        OperationMode::Decode { path, chunk_type } => {}
        OperationMode::Remove { path, chunk_type } => {
            let mut file = File::open(&path)?;

            let mut reader = BufReader::new(&file);
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;

            let mut png = Png::try_from(buf.as_ref())?;
            png.remove_first_chunk(&chunk_type)?;

            file.write_all(&png.as_bytes())?;
        }
        OperationMode::Print { path } => {}
    }
    Ok(())
}
