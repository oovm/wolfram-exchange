use crate::{Result, SupportedFormat};
use std::{fs, fs::File, io::Write, path::PathBuf};
use wolfram_wxf::{parse_json5, parse_toml, parse_yaml, WolframValue};

pub fn parse_file(path: &PathBuf, format: Option<String>) -> Result<WolframValue> {
    let s = fs::read_to_string(path)?;
    let suffix = match format {
        Some(s) => s,
        None => path.extension().unwrap_or_default().to_str().unwrap_or_default().to_string(),
    };
    let format = match suffix.to_lowercase().as_str() {
        "yml" | "yaml" => SupportedFormat::YAML,
        "json" | "json5" => SupportedFormat::JSON5,
        _ => SupportedFormat::TOML,
    };
    println!("Parsing the file {:?} as {:?}", path, format);
    match format {
        SupportedFormat::JSON5 => parse_json5(&s),
        SupportedFormat::TOML => parse_toml(&s),
        SupportedFormat::YAML => parse_yaml(&s),
        SupportedFormat::Pickle => unimplemented!(),
    }
}

pub fn write_to_file(path: &str, bytes: &[u8]) -> Result<()> {
    println!("Generating {:?}", path);
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    Ok(())
}
