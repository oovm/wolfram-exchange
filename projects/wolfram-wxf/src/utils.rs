use crate::{Error, SupportedFormat};
use std::{
    fs,
    fs::File,
    io::{ErrorKind, Write},
};
use wolfram_wxf::{
    utils::{parse_json, parse_toml, parse_yaml},
    WolframValue,
};

pub fn parse_file(path: &str, format: Option<&str>) -> Result<WolframValue, Error> {
    let s = fs::read_to_string(path)?;
    let suffix = match format {
        Some(s) => s,
        None => path.split('/').last()?.split('.').last()?,
    };
    let format = match suffix.to_lowercase().as_str() {
        "yml" | "yaml" => SupportedFormat::YAML,
        "json" => SupportedFormat::JSON,
        _ => SupportedFormat::TOML,
    };
    println!("Parsing the file {} as {:?}", path, format);
    match format {
        SupportedFormat::JSON => parse_json(&s).map_err(|_| Error::ParseFailed),
        SupportedFormat::TOML => parse_toml(&s).map_err(|_| Error::ParseFailed),
        SupportedFormat::YAML => parse_yaml(&s).map_err(|_| Error::ParseFailed),
        SupportedFormat::Pickle => unimplemented!(),
    }
}

pub fn write_to_file(path: &str, bytes: &[u8]) -> Result<(), Error> {
    println!("Generating {}", path);
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    Ok(())
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            ErrorKind::NotFound => Error::FileNotFound,
            ErrorKind::PermissionDenied => Error::PermissionDenied,
            _ => Error::UnknownIOError,
        }
    }
}

impl From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Self {
        Error::NullException
    }
}
