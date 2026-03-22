use EntryType::*;
use clap::builder::PossibleValuesParser;
use clap::{Arg, ArgAction, Command};
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let get_matches = Command::new("findr")
        .name("findr")
        .version("0.1.0")
        .author("Bthxtly <bthxtly@gmail.com>")
        .about("Rust findr")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .default_values(vec!["."])
                .action(ArgAction::Append)
                .help("paths to search into"),
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .short('n')
                .long("name")
                .action(ArgAction::Append)
                .num_args(0..)
                .help("Base of the file name matches shell pattern."),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .short('t')
                .long("type")
                .action(ArgAction::Append)
                .value_parser(PossibleValuesParser::new(["f", "d", "l"]))
                .num_args(0..)
                .help("Search for type TYPE"),
        )
        .get_matches();

    let paths = get_matches.get_many("paths").unwrap().cloned().collect();
    let names = get_matches
        .get_many::<String>("names")
        .into_iter()
        .flatten()
        .map(|name| Regex::new(name).map_err(|_| format!("Invalid --name \"{}\"", name)))
        .collect::<Result<_, _>>()?;
    let entry_types = get_matches
        .get_many::<String>("types")
        .into_iter()
        .flatten()
        .map(|typ| match typ.as_str() {
            "f" => File,
            "d" => Dir,
            "l" => Link,
            _ => unreachable!("Invalid type"),
        })
        .collect::<Vec<_>>();
    Ok(Config {
        paths,
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = |entry: &DirEntry| -> bool {
        config.entry_types.is_empty()
            || config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    Link => entry.file_type().is_symlink(),
                    Dir => entry.file_type().is_dir(),
                    File => entry.file_type().is_file(),
                })
    };
    let name_filter = |entry: &DirEntry| -> bool {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|entry| match entry {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        for entry in entries {
            println!("{}", entry);
        }
    }
    Ok(())
}
