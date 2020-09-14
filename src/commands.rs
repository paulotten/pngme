use std::io::prelude::*;
use std::fs::File;
use std::process;
use std::convert::TryFrom;
use std::str::FromStr;

use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

pub fn encode(filename: &str, chunk_type: &str, msg: &str, output_filename: &str) {
    let mut png = read_png_from_file(filename);

    let chunk_type = match ChunkType::from_str(chunk_type) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Invalid chunk type: {}", err);
            process::exit(1);
        },
    };
    
    let chunk = Chunk::new(chunk_type, msg.as_bytes().to_vec());

    png.append_chunk(chunk);

    write_file(output_filename, png.as_bytes().as_slice());
}

pub fn decode(filename: &str, chunk_type: &str) {
    let png = read_png_from_file(filename);

    let chunk = match png.chunk_by_type(chunk_type) {
        Some(c) => c,
        _ => {
            eprintln!("Chunk type `{}` not found", chunk_type);
            process::exit(1);
        },
    };

    let chunk_string = match chunk.data_as_string() {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error reading chunk data: {}", err);
            process::exit(1);
        }
    };

    println!("Chunk data: `{}`", chunk_string);
}

pub fn remove(filename: &str, chunk_type: &str) {
    let mut png = read_png_from_file(filename);

    match png.remove_chunk(chunk_type) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error removing chunk: {}", err);
            process::exit(1);
        },
    }

    write_file(filename, png.as_bytes().as_slice());
}

pub fn print(filename: &str) {
    let png = read_png_from_file(filename);

    println!("{}", png);
}

fn read_png_from_file(filename: &str) -> Png {
    let contents = read_file(filename);

    match Png::try_from(&contents[..]) {
        Ok(png) => png,
        Err(err) => {
            eprintln!("Error parsing PNG {:?}", err);
            process::exit(1);
        },
    }
}

fn read_file(filename: &str) -> Vec<u8> {
    let mut f = match File::open(filename) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error opening file `{}`: {:?}", filename, err);
            process::exit(1);
        }
    };

    let mut buffer = Vec::new();
    match f.read_to_end(&mut buffer) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error reading file `{}`: {:?}", filename, err);
            process::exit(1);
        }
    };

    buffer
}

fn write_file(filename: &str, data: &[u8]) {
    let mut f = match File::create(filename) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error creatubg file `{}`: {:?}", filename, err);
            process::exit(1);
        }
    };

    match f.write_all(data) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error writing to file `{}`: {:?}", filename, err);
            process::exit(1);
        }
    }
}
