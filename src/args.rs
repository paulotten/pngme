extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

use crate::*;

pub fn process_args() {
    let args = App::new("PNGme")
        .version("1.0")
        .author("Paul Otten <lightnica@yahoo.com>")
        .about("Hides secret messages in PNG files")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("encode")
            .about("Encodes (adds) a message to PNG file")
            .arg(Arg::with_name("FILE")
                .help("Input PNG file name")
                .required(true)
                .index(1)
            )
            .arg(Arg::with_name("CHUNK_TYPE")
                .help("Chunk type for message. Try \"RuSt\".")
                .required(true)
                .index(2)
            )
            .arg(Arg::with_name("MESSAGE")
                .help("The message")
                .required(true)
                .index(3)
            )
            .arg(Arg::with_name("OUTPUT_FILE")
                .help("[Optional] output png file name. Will default to overwriting FILE if not specified.")
                .index(4)
            )
        )
        .subcommand(SubCommand::with_name("decode")
            .about("Decodes (reads) a message from a PNG file")
            .arg(Arg::with_name("FILE")
                .help("PNG file name")
                .required(true)
                .index(1)
            )
            .arg(Arg::with_name("CHUNK_TYPE")
                .help("Chunk type")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name("remove")
            .about("Removed a message from a PNG file")
            .arg(Arg::with_name("FILE")
                .help("PNG file name")
                .required(true)
                .index(1)
            )
            .arg(Arg::with_name("CHUNK_TYPE")
                .help("Chunk type")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name("print")
            .about("Prints information about a PNG file")
            .arg(Arg::with_name("FILE")
                .help("PNG file name")
                .required(true)
                .index(1)
            )
        )
        .get_matches();

    match args.subcommand_name() {
        Some("encode") => {
            let sub_args = args.subcommand_matches("encode").unwrap();

            commands::encode(
                sub_args.value_of("FILE").unwrap(),
                sub_args.value_of("CHUNK_TYPE").unwrap(),
                sub_args.value_of("MESSAGE").unwrap(),
                // optional, defaults to FILE
                match sub_args.value_of("OUTPUT_FILE") {
                    Some(f) => f,
                    _ => sub_args.value_of("FILE").unwrap(),
                },
            );
        }
        Some("decode") => {
            let sub_args = args.subcommand_matches("decode").unwrap();

            commands::decode(
                sub_args.value_of("FILE").unwrap(),
                sub_args.value_of("CHUNK_TYPE").unwrap(),
            );
        }
        Some("remove") => {
            let sub_args = args.subcommand_matches("remove").unwrap();

            commands::remove(
                sub_args.value_of("FILE").unwrap(),
                sub_args.value_of("CHUNK_TYPE").unwrap(),
            );
        }
        Some("print") => {
            let sub_args = args.subcommand_matches("print").unwrap();

            commands::print(sub_args.value_of("FILE").unwrap());
        }
        _ => panic!("unknown subcommand"),
    }
}
