mod extract;

use anyhow::{Result, bail};
use csv::{ReaderBuilder, WriterBuilder};
use extract::{Extract, extract_bytes, extract_chars, extract_fields, parse_pos};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "FILES", default_value = "-", action=ArgAction::Append)]
    files: Vec<String>,

    #[arg(short, long, value_name = "DELIMITER", default_value = "\t")]
    delimiter: String,

    #[command(flatten)]
    extract: ArgsExtract,
    // #[arg(short='s', long="only-delimited", action=ArgAction::SetTrue)]
    // only_delimited: bool,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct ArgsExtract {
    #[arg(short, long, value_name = "FIELDS")]
    fields: Option<String>,

    #[arg(short, long, value_name = "BYTES")]
    bytes: Option<String>,

    #[arg(short, long, value_name = "CHARS")]
    chars: Option<String>,
}

pub fn run(args: Args) -> Result<()> {
    // validate delimiter
    if args.delimiter.len() != 1 {
        bail!(r#"--delimiter "{}" must be a single byte"#, args.delimiter);
    }
    let delimiter = *args.delimiter.as_bytes().first().unwrap();

    let extract = if let Some(fields) = args.extract.fields {
        Extract::Fields(parse_pos(fields)?)
    } else if let Some(bytes) = args.extract.bytes {
        Extract::Bytes(parse_pos(bytes)?)
    } else if let Some(chars) = args.extract.chars {
        Extract::Chars(parse_pos(chars)?)
    } else {
        unreachable!("Must have --fields, --bytes, or --chars");
    };

    for filename in &args.files {
        match open(filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => match &extract {
                Extract::Fields(field_pos) => {
                    let mut reader = ReaderBuilder::new()
                        .delimiter(delimiter)
                        .has_headers(false)
                        .from_reader(file);

                    let mut wtr = WriterBuilder::new()
                        .delimiter(delimiter)
                        .from_writer(io::stdout());

                    for record in reader.records() {
                        wtr.write_record(extract_fields(&record?, field_pos))?;
                    }
                }

                Extract::Bytes(byte_pos) => {
                    for line in file.lines() {
                        println!("{}", extract_bytes(&line?, byte_pos));
                    }
                }

                Extract::Chars(char_pos) => {
                    for line in file.lines() {
                        println!("{}", extract_chars(&line?, char_pos));
                    }
                }
            },
        }
    }
    Ok(())
}

fn open(file: &str) -> Result<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
