// TODO: make it works more like the GNU one
// Write my `tree` and support -l option like `ls`

use anyhow::Result;
use chrono::{DateTime, Local};
use clap::Parser;
use std::{
    fs::{self, metadata},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug, Parser)]
#[command(author, version, about = "Rust version of `ls`")]
struct Args {
    #[arg(
        default_value = ".",
        value_name = "FILE",
        help = "files and/or directories"
    )]
    paths: Vec<String>,

    #[arg(short, long, help = "use a long listing format")]
    long: bool,

    #[arg(short, long, help = "do not ignore entries starting with .")]
    all: bool,
}

fn run(args: Args) -> Result<()> {
    let paths = find_files(&args.paths, args.all)?;
    if args.long {
        println!("{}", format_output(&paths)?);
    } else {
        for path in paths {
            println!("{}", path.display());
        }
    }
    Ok(())
}

fn find_files(paths: &[String], show_hidden: bool) -> Result<Vec<PathBuf>> {
    let mut result = vec![];

    for path in paths {
        match fs::metadata(path) {
            Err(e) => eprintln!("{path}: {e}"),
            Ok(metadata) => {
                if metadata.is_file() {
                    result.push(PathBuf::from(path))
                } else if metadata.is_dir() {
                    result.extend(
                        fs::read_dir(path)?
                            .filter_map(Result::ok)
                            .map(|entry| entry.path())
                            .filter(|path| {
                                let is_hidden = path.file_name().is_some_and(|filename| {
                                    filename.to_string_lossy().starts_with('.')
                                });
                                show_hidden || !is_hidden
                            }),
                    )
                }
            }
        }
    }

    Ok(result)
}

fn format_output(paths: &[PathBuf]) -> Result<String> {
    //         1   2     3     4     5     6     7     8
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let data = metadata(path)?;
        let is_dir = if path.is_dir() { "d" } else { "-" };

        let permissions = format_mode(data.mode());

        let links = data.nlink().to_string();

        let user = get_user_by_uid(data.uid()).expect("User not found");
        let group = get_group_by_gid(data.gid()).expect("Group not found");

        let size = data.len().to_string();

        let modified_time = DateTime::<Local>::from(data.modified()?).format("%b %d %y %H:%M");

        table.add_row(
            Row::new()
                .with_cell(is_dir) // 1 "d" or "-"
                .with_cell(permissions) // 2 permissions
                .with_cell(links) // 3 number of links
                .with_cell(user.name().display()) // 4 user name
                .with_cell(group.name().display()) // 5 group name
                .with_cell(size) // 6 size
                .with_cell(modified_time) // 7 modification
                .with_cell(path.display()), // 8 path
        );
    }

    Ok(format!("{table}"))
}

/// Given a file mode in octal format like 0o751,
/// return a string like "rwxr-x--x"
fn format_mode(mode: u32) -> String {
    let mut permissions = ['-'; 9];
    if mode & 0o400 != 0 {
        permissions[0] = 'r';
    }
    if mode & 0o200 != 0 {
        permissions[1] = 'w';
    }
    if mode & 0o010 != 0 {
        permissions[2] = 'x';
    }
    if mode & 0o040 != 0 {
        permissions[3] = 'r';
    }
    if mode & 0o020 != 0 {
        permissions[4] = 'w';
    }
    if mode & 0o010 != 0 {
        permissions[5] = 'x';
    }
    if mode & 0o004 != 0 {
        permissions[6] = 'r';
    }
    if mode & 0o002 != 0 {
        permissions[7] = 'w';
    }
    if mode & 0o001 != 0 {
        permissions[8] = 'x';
    }
    permissions.iter().collect()
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_files() {
        // Find all nonhidden entries in a directory
        let res = find_files(&["tests/inputs".to_string()], false);

        assert!(res.is_ok());

        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();

        assert_eq!(
            filenames,
            [
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );

        // Find all entries in a directory
        let res = find_files(&["tests/inputs".to_string()], true);

        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );

        // Any existing file should be found even if hidden
        let res = find_files(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);

        // Test multiple path arguments
        let res = find_files(
            &[
                "tests/inputs/bustle.txt".to_string(),
                "tests/inputs/dir".to_string(),
            ],
            false,
        );
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
        );
    }

    #[test]
    fn test_find_files_hidden() {
        let res = find_files(&["tests/inputs".to_string()], true);

        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );
    }

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o421), "r---w---x");
    }
}
