use clap::{Arg, ArgAction, Command};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number: bool,
    number_nonblank: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .name("catr")
        .version("0.1.0")
        .author("Bthxtly <bthxtly@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .action(ArgAction::Append)
                .help("Input file(s)")
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .action(ArgAction::SetTrue)
                .help("Print line number"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .action(ArgAction::SetTrue)
                .help("Print line number only for unblank lines"),
        )
        .get_matches();

    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .cloned()
        .collect();

    Ok(Config {
        files,
        number: matches.get_flag("number"),
        number_nonblank: matches.get_flag("number_nonblank"),
    })
}

fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for file in config.files {
        match open(&file) {
            Err(e) => eprintln!("Failed to open {file}: {e}"),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;

                    if config.number {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }

    Ok(())
}
