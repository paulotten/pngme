use crate::chunk_type::ChunkType;
use std::convert::TryFrom;
use crate::{Error, Result};

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Chunk::calc_crc(&chunk_type, &data);
        Chunk {
            length: data.len() as u32,
            chunk_type: chunk_type,
            data: data,
            crc: crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let mut s = String::new();

        for c in self.data.iter() {
            s.push(*c as char);
        }

        Ok(s)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        // length
        bytes.extend(self.length.to_be_bytes().iter());

        // chunk type
        bytes.extend(self.chunk_type.bytes().iter());

        // chunk data
        bytes.extend(self.data.iter());

        // crc
        bytes.extend(self.crc.to_be_bytes().iter());

        bytes
    }

    /*
    Scans an array of bytes assumed to contain at least one chunk
    Returns a Result containing
        the total length of the first chunk in bytes
        OR an error

    Used by `impl TryFrom<&[u8]> for Png`
    */
    pub fn get_total_length_from_bytes(arr: &[u8]) -> Result<u32> {
        let overhead: u32 = 12; // length (4 bytes) + type (4 bytes) + crc (4 bytes)

        let mut iter = arr.iter();
        let mut len: u32 = 0;

        for _i in 0..4 {
            len *= 256;
            len += match iter.next() {
                Some(i) => *i as u32,
                None => return Err("ran out of bytes reading length"),
            };
        }

        Ok(overhead + len)
    }

    fn calc_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let check_me = [&ChunkType::bytes(&chunk_type)[..], &data[..]].concat();
        crc::crc32::checksum_ieee(&check_me)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(arr: &[u8]) -> Result<Self> {
        let mut iter = arr.iter();

        // length, 4 bytes
        let mut len: u32 = 0;

        for _i in 0..4 {
            len *= 256;
            len += match iter.next() {
                Some(i) => *i as u32,
                None => return Err("ran out of bytes reading length"),
            };
        }

        // type, 4 bytes
        let mut type_arr: [u8; 4] = [0; 4];

        for i in 0..4 {
            type_arr[i] = match iter.next() {
                Some(i) => *i,
                None => return Err("ran out of bytes reading chunk type"),
            };
        }

        let chunk_type = match ChunkType::try_from(type_arr) {
            Ok(c) => c,
            Err(_) => return Err("error creating chunk type"),
        };

        // data, length bytes
        let mut data: Vec<u8> = vec![];

        for _i in 0..len {
            data.push(match iter.next() {
                Some(i) => *i,
                None => return Err("ran out of bytes reading chunk data"),
            });
        }

        // crc, 4 bytes
        let mut crc: u32 = 0;

        for _i in 0..4 {
            crc *= 256;
            crc += match iter.next() {
                Some(i) => *i as u32,
                None => return Err("ran out of bytes reading crc"),
            };
        }

        // validate crc
        if Chunk::calc_crc(&chunk_type, &data) != crc {
            return Err("invalid crc");
        }

        Ok(Chunk {
            length: len,
            chunk_type: chunk_type,
            data: data,
            crc: crc,
        })
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.data_as_string() {
            Ok(s) => s,
            Err(_) => panic!(),
        };

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
