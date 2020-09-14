use crate::{Error, Result};

#[derive(Debug)]
pub struct ChunkType(u8, u8, u8, u8);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.0 & (ChunkType::FIFTH_BIT) == 0
    }

    pub fn is_public(&self) -> bool {
        self.1 & (ChunkType::FIFTH_BIT) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.2 & (ChunkType::FIFTH_BIT) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.3 & (ChunkType::FIFTH_BIT) != 0
    }

    const FIFTH_BIT: u8 = 0b0010_0000;

    fn byte_is_valid(b: u8) -> bool {
        (b >= 'a' as u8 && b <= 'z' as u8) || (b >= 'A' as u8 && b <= 'Z' as u8)
    }

    fn from_arr(arr: [u8; 4]) -> Result<Self> {
        if !(ChunkType::byte_is_valid(arr[0])
            && ChunkType::byte_is_valid(arr[1])
            && ChunkType::byte_is_valid(arr[2])
            && ChunkType::byte_is_valid(arr[3]))
        {
            return Err("Invalid chunk byte value");
        }

        Ok(ChunkType(arr[0], arr[1], arr[2], arr[3]))
    }
}

impl std::convert::TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(arr: [u8; 4]) -> Result<Self> {
        ChunkType::from_arr(arr)
    }
}

impl std::str::FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let arr = s.as_bytes();

        if arr.len() != 4 {
            return Err("Invalid chunk length");
        }

        let mut b: [u8; 4] = [0, 0, 0, 0];
        b.copy_from_slice(&arr[0..4]);

        ChunkType::from_arr(b)
    }
}

impl std::cmp::PartialEq for ChunkType {
    fn eq(&self, other: &ChunkType) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}

impl std::fmt::Display for ChunkType {
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
