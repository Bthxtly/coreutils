use anyhow::{Result, anyhow};
use clap::{ArgAction, Parser};
use regex::{Regex, RegexBuilder};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    mem,
};

#[derive(Debug, Parser)]
#[command(
    author = "Bthxtly <bthxtly@gmail.com>",
    version = "0.1.0",
    about = "Rust version of `grep`"
)]
struct Args {
    #[arg(value_name = "PATTERNS")]
    #[arg(help = "Search pattern")]
    pattern: String,

    #[arg(value_name = "FILE", default_value = "-")]
    #[arg(help = "Input file(s)")]
    files: Vec<String>,

    #[arg(short, long="ignore-case", action=ArgAction::SetTrue)]
    #[arg(help = "Ignore case distinctions in patterns and input data")]
    insensitive: bool,

    #[arg(short, long, action=ArgAction::SetTrue)]
    #[arg(
        help = "Read all files under each directory, recursively, following symbolic links only if they are on the command line"
    )]
    recursive: bool,

    #[arg(short, long, action=ArgAction::SetTrue)]
    #[arg(
        help = "Suppress normal output; instead print a count of matching lines for each input file"
    )]
    count: bool,

    #[arg(short='v', long="invert-match", action=ArgAction::SetTrue)]
    #[arg(help = "Invert the sense of matching, to select non-matching lines")]
    invert: bool,
}

fn run(args: Args) -> Result<()> {
    let pattern = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.insensitive)
        .build()
        .map_err(|_| anyhow!(r#"Invalid pattern "{}""#, args.pattern))?;
    let entries = find_files(&args.files, args.recursive);
    let num_files = entries.len();
    let print = |filename: &str, val: &str| {
        if num_files > 1 {
            print!("{filename}:{val}");
        } else {
            print!("{val}");
        }
    };

    for entry in entries {
        match entry {
            Err(e) => eprintln!("{}", e),
            Ok(filename) => match open(&filename) {
                Err(e) => eprintln!("{}: {}", filename, e),
                Ok(file) => match find_lines(file, &pattern, args.invert) {
                    Err(e) => eprintln!("{e}"),
                    Ok(matches) => {
                        if args.count {
                            print(&filename, &format!("{}\n", matches.len()));
                        } else {
                            for line in &matches {
                                print(&filename, line);
                            }
                        }
                    }
                },
            },
        }
    }
    Ok(())
}

fn open(file: &str) -> Result<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}

fn find_files(paths: &[String], recursive: bool) -> Vec<Result<String>> {
    let mut result = vec![];

    for path in paths {
        match path.as_str() {
            "-" => result.push(Ok(path.to_string())),
            _ => match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if recursive {
                            for entry in walkdir::WalkDir::new(path)
                                .into_iter()
                                .flatten()
                                .filter(|e| e.file_type().is_file())
                            {
                                result.push(Ok(entry.path().display().to_string()));
                            }
                        } else {
                            result.push(Err(anyhow!("{path} is a directory")))
                        }
                    } else if metadata.is_file() {
                        result.push(Ok(path.to_string()));
                    }
                }
                Err(e) => result.push(Err(anyhow!("{path}: {e}"))),
            },
        }
    }

    result
}

fn find_lines(mut file: impl BufRead, pattern: &Regex, intevrt: bool) -> Result<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if pattern.is_match(&line) ^ intevrt {
            matches.push(mem::take(&mut line));
        }
        line.clear();
    }

    Ok(matches)
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
    use rand::{Rng, distributions::Alphanumeric};

    #[test]
    fn test_find_files() {
        // Verify that the function finds a file known to exist
        let files = find_files(&["./tests/inputs/fox.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/fox.txt");

        // The function should reject a directory without the recursive option
        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        // Verify the function recurses to find four files in the directory
        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 4);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/bustle.txt",
                "./tests/inputs/empty.txt",
                "./tests/inputs/fox.txt",
                "./tests/inputs/nobody.txt",
            ]
        );

        // Generate a random string to represent a nonexistent file
        let bad: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Verify that the function returns the bad file as an error
        let files = find_files(&[bad], false);
        assert_eq!(files.len(), 1);
        assert!(files[0].is_err());
    }

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // The pattern _or_ should match the one line, "Lorem"
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        // When inverted, the function should match the other two lines
        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // The two lines "Lorem" and "DOLOR" should match
        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // When inverted, the one remaining line should match
        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }
}
