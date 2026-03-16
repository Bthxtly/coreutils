use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::{Arg, ArgAction, Command, value_parser};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let get_matches = Command::new("headr")
        .name("headr")
        .version("0.1.0")
        .author("Bthxtly <bthxtly@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .default_value("-")
                .action(ArgAction::Append)
                .help("Input file(s)"),
        )
        .arg(
            Arg::new("lines")
            .short('n')
            .value_name("LINES")
            .default_value("10")
            .value_parser(value_parser!(usize))
            .help("print the first NUM lines instead of the first 10; with the leading '-', print all but the last NUM lines of each file")
            .conflicts_with("bytes")
            )
        .arg(
            Arg::new("bytes")
            .short('c')
            .value_name("BYTES")
            .value_parser(value_parser!(usize))
            .help("print the first NUM bytes of each file; with the leading '-', print all but the last NUM bytes of each file")
            )
        .get_matches();

    let files = get_matches
        .get_many::<String>("files")
        .unwrap()
        .cloned()
        .collect();
    let lines = *get_matches.get_one("lines").unwrap();
    let bytes = get_matches.get_one::<usize>("bytes").copied();

    Ok(Config {
        files,
        lines,
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut is_first = true;
    let file_num = config.files.len();

    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(mut file) => {
                if file_num > 1 {
                    print_heading(&mut is_first, &filename);
                }

                if let Some(bytes) = config.bytes {
                    let mut handle = file.take(bytes as u64);
                    let mut buffer = vec![0; bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn print_heading(is_first: &mut bool, file: &str) {
    if !*is_first {
        println!();
    } else {
        *is_first = false;
    }
    println!("==> {file} <==");
}

fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
