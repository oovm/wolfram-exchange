pub mod utils;

use crate::utils::{parse_file, write_to_file};
use clap::{App, Arg};

#[derive(Debug)]
pub enum SupportedFormat {
    JSON5,
    TOML,
    YAML,
    Pickle,
}

#[derive(Debug)]
pub enum Error {
    NullException,
    FileNotFound,
    PermissionDenied,
    UnknownIOError,
    ParseFailed,
}

fn main() -> Result<(), Error> {
    #[rustfmt::skip]
        let matches = App::new("Wolfram Format Converter")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
        .required(true)
            .index(1))
        .arg(Arg::with_name("Text")
            .short("t")
            .long("text")
            .help("Generates m file")
            .multiple(true)
            .takes_value(false))
        .arg(Arg::with_name("Binary")
            .short("b")
            .long("binary")
            .help("Generates wxf file")
            .multiple(true)
            .takes_value(false))
        .arg(Arg::with_name("Compress")
            .short("c")
            .long("compress")
            .help("Generates mx file")
            .multiple(true)
            .takes_value(false))
        .arg(Arg::with_name("File Format")
            .short("f")
            .long("format")
            .value_name("Format")
            .help("Sets the input file format")
            .takes_value(true))
        .get_matches();
    let input = matches.value_of("INPUT").unwrap();

    let value = parse_file(input, matches.value_of("File Format"))?;
    match matches.occurrences_of("Text") {
        0 => (),
        _ => write_to_file(&format!("{}.m", input), value.to_string().as_bytes())?,
    }
    match matches.occurrences_of("Binary") {
        0 => (),
        _ => write_to_file(&format!("{}.wxf", input), &value.to_bytes())?,
    }
    match matches.occurrences_of("Compress") {
        0 => (),
        _ => write_to_file(&format!("{}.mx", input), &value.to_compressed())?,
    };
    Ok(())
}
