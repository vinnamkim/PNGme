use std::{fmt::Display, string::FromUtf8Error};

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;

#[derive(PartialEq, Debug)]
pub enum ChunkError {
    CreationError,
}

pub struct Chunk {
    _chunk_type: ChunkType,
    _data: Vec<u8>,
    _length: usize,
    _crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let bytes = Chunk::_as_bytes(&chunk_type, data.as_ref());
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let length = data.len();

        Chunk {
            _chunk_type: chunk_type,
            _data: data,
            _length: length,
            _crc: crc.checksum(bytes.as_slice()),
        }
    }
    pub fn length(&self) -> u32 {
        self._length as u32
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self._chunk_type
    }
    pub fn data(&self) -> &[u8] {
        self._data.as_ref()
    }
    pub fn crc(&self) -> u32 {
        self._crc
    }
    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self._data.clone())
    }
    fn _as_bytes(chunk_type: &ChunkType, data: &[u8]) -> Vec<u8> {
        chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect()
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes = Chunk::_as_bytes(self.chunk_type(), self.data());
        let crc = self.crc();
        self.length()
            .to_be_bytes()
            .iter()
            .chain(bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(arr: &[u8]) -> Result<Self, Self::Error> {
        let data_length = u32::from_be_bytes(arr[0..4].try_into().unwrap()) as usize;
        let chunk_type_bytes: [u8; 4] = arr[4..8].try_into().unwrap();
        let mut message_bytes = Vec::new();
        for i in 0..data_length {
            let idx = 8 + i;
            message_bytes.push(arr[idx]);
        }

        let crc = u32::from_be_bytes([
            arr[8 + data_length + 0],
            arr[8 + data_length + 1],
            arr[8 + data_length + 2],
            arr[8 + data_length + 3],
        ]);

        if let Ok(x) = ChunkType::try_from(chunk_type_bytes) {
            let chunk = Chunk::new(x, message_bytes);
            if chunk.crc() == crc {
                Ok(chunk)
            } else {
                Err(ChunkError::CreationError)
            }
        } else {
            Err(ChunkError::CreationError)
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data_as_string().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
