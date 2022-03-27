pub mod utils;

use crate::utils::{parse_file, write_to_file};
use clap::Parser;
use std::path::PathBuf;
use wolfram_wxf::Result;

#[derive(Debug)]
pub enum SupportedFormat {
    JSON5,
    TOML,
    YAML,
    Pickle,
}

/// The main entry point for the program.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct WolframExchange {
    /// Sets the input file to use
    #[clap(parse(from_os_str), value_name = "FILE")]
    input: PathBuf,

    /// Sets a custom config file
    #[clap(short = 't', long = "text")]
    generate_text: bool,
    /// Sets a custom config file
    #[clap(short = 'b', long = "binary")]
    generate_binary: bool,
    /// Sets a custom config file
    #[clap(short = 'c', long = "compress")]
    generate_compress: bool,
    /// Sets the input file format
    #[clap(short, long)]
    format: Option<String>,
}

fn main() -> Result<()> {
    let app = WolframExchange::parse();
    println!("{:#?}", app);
    let value = parse_file(&app.input, app.format)?;
    if app.generate_text {
        write_to_file(&app.input.join(".m"), value.to_string().as_bytes())?
    }
    if app.generate_binary {
        write_to_file(&app.input.join(".wxf"), value.to_string().as_bytes())?
    }
    if app.generate_compress {
        write_to_file(&app.input.join(".mx"), value.to_string().as_bytes())?
    }
    Ok(())
}
