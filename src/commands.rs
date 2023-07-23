use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use clap::Parser;
use derive_more::{Display, Error};

use crate::args::PngMeArgs;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::{Png, PngError};

#[derive(PartialEq, Debug, Display, Error)]
pub enum CommandError {
    NotExistingChunkType,
}

pub(crate) fn cli() -> Result<(), Box<dyn Error>> {
    let args = PngMeArgs::parse();

    match args {
        PngMeArgs::Encode(args) => {
            if args.filepath.as_path().exists() {
                let contents = std::fs::read(args.filepath.as_path())?;
                let mut png = Png::try_from(contents.as_ref())?;
                let chunk = parse_chunk(args.chunk_type, args.data)?;
                png.append_chunk(chunk);

                let to_write = png.as_bytes();
                std::fs::write(args.filepath.as_path(), to_write.as_slice())?;
                Ok(())
            } else {
                let mut file = File::create(args.filepath.as_path())?;
                let png = Png::from_chunks(vec![parse_chunk(args.chunk_type, args.data)?]);

                file.write_all(png.as_bytes().as_ref())?;
                Ok(())
            }
        }
        PngMeArgs::Decode(args) => {
            let contents = std::fs::read(args.filepath.as_path())?;
            let png = Png::try_from(contents.as_ref())?;

            let chunk = png.chunk_by_type(args.chunk_type.as_str());

            match chunk {
                Some(x) => {
                    println!("{}", x);
                    Ok(())
                }
                None => Err(Box::new(CommandError::NotExistingChunkType)),
            }
        }
        PngMeArgs::Remove(args) => {
            if args.filepath.as_path().exists() {
                let contents = std::fs::read(args.filepath.as_path())?;
                let mut png = Png::try_from(contents.as_ref())?;
                png.remove_chunk(args.chunk_type.as_str())?;

                let to_write = png.as_bytes();
                std::fs::write(args.filepath.as_path(), to_write.as_slice())?;
                Ok(())
            } else {
                Err(Box::new(CommandError::NotExistingChunkType))
            }
        }
        PngMeArgs::Print(args) => {
            println!("Print: {}", args.filepath.as_path().display());
            let contents = std::fs::read(args.filepath.as_path())?;
            let png = Png::try_from(contents.as_ref())?;
            println!("{}", png);
            Ok(())
        }
    }
}

fn parse_chunk(chunk_type: String, data: String) -> Result<Chunk, Box<dyn Error>> {
    let chunk_type = ChunkType::from_str(chunk_type.as_str())?;
    let chunk = Chunk::new(chunk_type, data.bytes().collect());
    Ok(chunk)
}
