mod fileinfo;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let get_matches = Command::new("wcr")
        .name("wcr")
        .version("0.1.0")
        .author("Bthxtly <bthxtly@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .default_value("-")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .action(ArgAction::SetTrue)
                .help("print the newline counts"),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .action(ArgAction::SetTrue)
                .help("print the word counts"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .action(ArgAction::SetTrue)
                .help("print the byte counts")
                .conflicts_with("chars"),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .action(ArgAction::SetTrue)
                .help("print the character counts"),
        )
        .get_matches();

    let files = get_matches.get_many("files").unwrap().cloned().collect();
    let mut lines = get_matches.get_flag("lines");
    let mut words = get_matches.get_flag("words");
    let mut bytes = get_matches.get_flag("bytes");
    let chars = get_matches.get_flag("chars");

    // if no option selected, use lines, words and bytes by default
    if [lines, words, bytes, chars].iter().all(|v| !v) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => {
                let info = fileinfo::count(file)?;
                let filename = if filename == "-" {
                    ""
                } else {
                    &format!(" {filename}")
                };
                println!(
                    "{}{}{}{}{}",
                    format_field(info.num_lines, config.lines),
                    format_field(info.num_words, config.words),
                    format_field(info.num_bytes, config.bytes),
                    format_field(info.num_chars, config.chars),
                    filename
                );
                total_lines += info.num_lines;
                total_words += info.num_words;
                total_bytes += info.num_bytes;
                total_chars += info.num_chars;
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars),
        );
    }

    Ok(())
}

fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::fileinfo::*;
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
