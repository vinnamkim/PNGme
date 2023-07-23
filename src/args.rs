use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(
    name = "pngme",
    bin_name = "pngme",
    about = "A command line program that lets you hide secret messages in PNG files."
)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(clap::Args, Debug)]
#[command(author, version, about = "Encode a message into a PNG file", long_about = None)]
pub struct EncodeArgs {
    filepath: PathBuf,
    chunk_type: String,
    data: String,
}

#[derive(clap::Args, Debug)]
#[command(author, version, about="Decode a message stored in a PNG file", long_about = None)]
pub struct DecodeArgs {
    filepath: PathBuf,
    chunk_type: String,
}

#[derive(clap::Args, Debug)]
#[command(author, version, about="Remove a message from a PNG file", long_about = None)]
pub struct RemoveArgs {
    filepath: PathBuf,
    chunk_type: String,
}

#[derive(clap::Args, Debug)]
#[command(author, version, about="Print a list of PNG chunks that can be searched for messages", long_about = None)]
pub struct PrintArgs {
    filepath: PathBuf,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    pub fn test_encode_args() {
        let result = PngMeArgs::parse_from([
            "pngme",
            "encode",
            "./dice.png",
            "ruSt",
            "This is a secret message!",
        ]);

        if let PngMeArgs::Encode(x) = result {
            let actual = x.filepath;
            let expect = PathBuf::from_str("./dice.png").unwrap();
            assert_eq!(actual, expect);
            assert_eq!(x.chunk_type, "ruSt");
            assert_eq!(x.data, "This is a secret message!");
        } else {
            panic!();
        }
    }

    #[test]
    pub fn test_decode_args() {
        let result = PngMeArgs::parse_from(["pngme", "decode", "./dice.png", "ruSt"]);

        if let PngMeArgs::Decode(x) = result {
            let actual = x.filepath;
            let expect = PathBuf::from_str("./dice.png").unwrap();
            assert_eq!(actual, expect);
            assert_eq!(x.chunk_type, "ruSt");
        } else {
            panic!();
        }
    }

    #[test]
    pub fn test_remove_args() {
        let result = PngMeArgs::parse_from(["pngme", "remove", "./dice.png", "ruSt"]);

        if let PngMeArgs::Remove(x) = result {
            let actual = x.filepath;
            let expect = PathBuf::from_str("./dice.png").unwrap();
            assert_eq!(actual, expect);
            assert_eq!(x.chunk_type, "ruSt");
        } else {
            panic!();
        }
    }

    #[test]
    pub fn test_print_args() {
        let result = PngMeArgs::parse_from(["pngme", "print", "./dice.png"]);

        if let PngMeArgs::Print(x) = result {
            let actual = x.filepath;
            let expect = PathBuf::from_str("./dice.png").unwrap();
            assert_eq!(actual, expect);
        } else {
            panic!();
        }
    }
}
