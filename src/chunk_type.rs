use std::{fmt::Display, str::FromStr};

use derive_more::{Display, Error};

#[derive(PartialEq, Debug, Display, Error)]
pub enum ChunkTypeError {
    BytesLengthError,
    NotASCIILetters,
}

#[derive(PartialEq, Debug)]
pub struct ChunkType(u8, u8, u8, u8);

const CHECK_BIT: u8 = 32;

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
    pub fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() || !self.is_valid_ascii() {
            false
        } else {
            true
        }
    }
    pub fn is_critical(&self) -> bool {
        if (self.0 & CHECK_BIT) == 0 {
            true
        } else {
            false
        }
    }
    pub fn is_public(&self) -> bool {
        if (self.1 & CHECK_BIT) == 0 {
            true
        } else {
            false
        }
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        if (self.2 & CHECK_BIT) == 0 {
            true
        } else {
            false
        }
    }
    pub fn is_safe_to_copy(&self) -> bool {
        if (self.3 & CHECK_BIT) > 0 {
            true
        } else {
            false
        }
    }
    pub fn is_valid_ascii(&self) -> bool {
        for u in self.bytes().iter() {
            let c = (*u) as char;
            if 'A' <= c && c <= 'Z' {
                continue;
            } else if 'a' <= c && c <= 'z' {
                continue;
            } else {
                return false;
            }
        }
        true
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let v = ChunkType(value[0], value[1], value[2], value[3]);
        if v.is_valid_ascii() {
            Ok(v)
        } else {
            Err(ChunkTypeError::NotASCIILetters)
        }
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.as_bytes();
        if value.len() == 4 {
            let arr = [value[0], value[1], value[2], value[3]];
            ChunkType::try_from(arr)
        } else {
            Err(ChunkTypeError::BytesLengthError)
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.0 as char, self.1 as char, self.2 as char, self.3 as char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
