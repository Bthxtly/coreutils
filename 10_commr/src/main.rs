// TODO: add more options

use anyhow::{Result, anyhow, bail};
use clap::{ArgAction, Parser};
use std::{
    cmp::Ordering::*,
    fs::File,
    io::{BufRead, BufReader, stdin},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_name = "FILE1")]
    file1: String,

    #[arg(value_name = "FILE2")]
    file2: String,

    #[arg(short='1', action=ArgAction::SetFalse, help="suppress column 1(lines unique to FILE1)")]
    show_col1: bool,

    #[arg(short='2', action=ArgAction::SetFalse, help="suppress column 2(lines unique to FILE2)")]
    show_col2: bool,

    #[arg(short='3', action=ArgAction::SetFalse, help="suppress column 3(lines that appear in both files)")]
    show_col3: bool,

    #[arg(short, action=ArgAction::SetTrue, help="case insensitive comparison of lines")]
    insensitive: bool,

    #[arg(
        short,
        long = "output-delimiter",
        value_name = "STR",
        default_value = "\t",
        help = "separate columns with STR"
    )]
    delimiter: String,
}

enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

fn run(args: Args) -> Result<()> {
    // die if both files are stdin
    if &args.file1 == "-" && &args.file2 == "-" {
        bail!(r#"Both input files cannot be STDIN ("-")"#);
    }

    let case = |line: String| {
        if args.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let print = |col: Column| {
        let mut columns = vec![];
        match col {
            Column::Col1(val) => {
                if args.show_col1 {
                    columns.push(val);
                }
            }
            Column::Col2(val) => {
                if args.show_col2 {
                    if args.show_col1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
            Column::Col3(val) => {
                if args.show_col3 {
                    if args.show_col1 {
                        columns.push("");
                    }
                    if args.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        };

        if !columns.is_empty() {
            println!("{}", columns.join(&args.delimiter));
        }
    };

    let mut lines1 = open(&args.file1)?.lines().map_while(Result::ok).map(case);
    let mut lines2 = open(&args.file2)?.lines().map_while(Result::ok).map(case);

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                Equal => {
                    print(Column::Col3(val1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Less => {
                    print(Column::Col1(val1));
                    line1 = lines1.next();
                }
                Greater => {
                    print(Column::Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val), None) => {
                print(Column::Col1(val));
                line1 = lines1.next();
            }
            (None, Some(val)) => {
                print(Column::Col2(val));
                line2 = lines2.next();
            }
            _ => (),
        }
    }

    Ok(())
}

fn open(file: &str) -> Result<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(file).map_err(|e| anyhow!("{file}: {e}"))?,
        ))),
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
