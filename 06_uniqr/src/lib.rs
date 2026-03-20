use clap::{Arg, ArgAction, Command};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let get_matches = Command::new("uniqr")
        .name("uniqr")
        .version("0.1.0")
        .author("Bthxtly <bthxtly@gmail.com>")
        .about("Rust uniq")
        .arg(
            Arg::new("in_file")
                .value_name("INPUT")
                .default_value("-")
                .help("Input file"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUTPUT")
                .help("Output file"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("prefix lines by the number of occurrences"),
        )
        .get_matches();

    let in_file = get_matches.get_one("in_file").cloned().unwrap();
    let out_file = get_matches.get_one("out_file").cloned();
    let count = get_matches.get_flag("count");
    Ok(Config {
        in_file,
        out_file,
        count,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", &config.in_file, e))?;
    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        }
        Ok(())
    };

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(count, &previous)?;

    Ok(())
}

fn open(file: &str) -> MyResult<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
