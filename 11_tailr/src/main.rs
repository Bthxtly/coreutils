// TODO: implement more options

use anyhow::{Result, anyhow, bail};
use clap::{ArgAction, Parser};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_name = "FILE", required = true)]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long,
        value_name = "NUM",
        default_value = "10",
        conflicts_with("bytes"),
        help = "output the last NUM lines"
    )]
    lines: String,

    #[arg(
        short = 'c',
        long,
        value_name = "NUM",
        help = "output the last NUM bytes"
    )]
    bytes: Option<String>,

    #[arg(short, long, action=ArgAction::SetTrue, help="never output headers giving file names")]
    quiet: bool,
}

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

fn parse_num(val: String) -> Result<TakeValue> {
    match val.as_str() {
        "+0" => Ok(TakeValue::PlusZero),
        _ => {
            let signs: &[char] = &['+', '-'];
            let res = if val.starts_with(signs) {
                val.parse()
            } else {
                val.parse().map(i64::wrapping_neg)
            };

            if let Ok(num) = res {
                Ok(TakeValue::TakeNum(num))
            } else {
                bail!(val);
            }
        }
    }
}

fn run(args: Args) -> Result<()> {
    let lines = parse_num(args.lines).map_err(|e| anyhow!("illegal line count -- {e}"))?;

    let bytes = args
        .bytes
        .map(parse_num)
        .transpose()
        .map_err(|e| anyhow!("illegal byte count -- {e}"))?;

    let mut is_first = true;
    for filename in &args.files {
        match File::open(filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => {
                if !args.quiet && args.files.len() > 1 {
                    print_heading(&mut is_first, filename);
                }

                let (total_lines, total_bytes) = count_lines_bytes(filename)?;
                let mut file = BufReader::new(file);
                if let Some(bytes) = &bytes {
                    print_bytes(&mut file, bytes, total_bytes)?;
                } else {
                    print_lines(&mut file, &lines, total_lines)?;
                }
            }
        }
    }
    Ok(())
}

fn print_heading(is_first: &mut bool, filename: &str) {
    if !*is_first {
        println!();
    } else {
        *is_first = false;
    }
    println!("==> {filename} <==");
}

fn count_lines_bytes(filename: &str) -> Result<(i64, i64)> {
    let mut file = BufReader::new(File::open(filename)?);
    let mut line = String::new();
    let mut total_lines: i64 = 0;
    let mut total_bytes: i64 = 0;

    loop {
        let byte = file.read_line(&mut line)?;
        if byte == 0 {
            break;
        }
        total_lines += 1;
        total_bytes += byte as i64;
        line.clear();
    }

    Ok((total_lines, total_bytes))
}

fn get_start_index(take_val: &TakeValue, total: i64) -> Option<u64> {
    match *take_val {
        TakeValue::PlusZero => {
            if total == 0 {
                None
            } else {
                Some(0)
            }
        }
        TakeValue::TakeNum(num) => {
            if num == 0 || total == 0 || num > total {
                None
            } else if num > 0 {
                Some((num - 1) as u64)
            } else {
                // num < 0
                let res = num + total;
                Some(if res < 0 { 0 } else { res as u64 })
            }
        }
    }
}

fn print_lines(mut file: impl BufRead, num_lines: &TakeValue, total_lines: i64) -> Result<()> {
    if let Some(start_index) = get_start_index(num_lines, total_lines) {
        let mut line_num = 0;
        let mut buf = String::new();
        loop {
            let bytes_read = file.read_line(&mut buf)?;
            if bytes_read == 0 {
                break;
            }
            if line_num >= start_index {
                print!("{buf}");
            }
            line_num += 1;
            buf.clear();
        }
    }

    Ok(())
}

fn print_bytes(mut file: impl Read + Seek, num_bytes: &TakeValue, total_bytes: i64) -> Result<()> {
    if let Some(start_index) = get_start_index(num_bytes, total_bytes) {
        file.seek(std::io::SeekFrom::Start(start_index))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        if !buf.is_empty() {
            print!("{}", String::from_utf8_lossy(&buf));
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::TakeValue::*;
    use super::*;

    #[test]
    fn test_parse_num() {
        // All integers should be interpreted as negative numbers
        let res = parse_num("3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // A leading "+" should result in a positive number
        let res = parse_num("+3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(3));

        // An explicit "-" value should result in a negative number
        let res = parse_num("-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // Zero is zero
        let res = parse_num("0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(0));

        // Plus zero is special
        let res = parse_num("+0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PlusZero);

        // Test boundaries
        let res = parse_num(i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num((i64::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(format!("+{}", i64::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MAX));

        let res = parse_num(i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));

        // A floating-point value is invalid
        let res = parse_num("3.14".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        // Any non-integer string is invalid
        let res = parse_num("foo".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }

    #[test]
    fn test_count_lines_bytes() {
        let res = count_lines_bytes("tests/inputs/one.txt");
        assert!(res.is_ok());
        let (lines, bytes) = res.unwrap();
        assert_eq!(lines, 1);
        assert_eq!(bytes, 24);

        let res = count_lines_bytes("tests/inputs/twelve.txt");
        assert!(res.is_ok());
        let (lines, bytes) = res.unwrap();
        assert_eq!(lines, 12);
        assert_eq!(bytes, 63);
    }

    #[test]
    fn test_get_start_index() {
        // +0 from an empty file (0 lines/bytes) returns None
        assert_eq!(get_start_index(&PlusZero, 0), None);

        // +0 from a nonempty file returns an index that
        // is one less than the number of lines/bytes
        assert_eq!(get_start_index(&PlusZero, 1), Some(0));

        // Taking 0 lines/bytes returns None
        assert_eq!(get_start_index(&TakeNum(0), 1), None);

        // Taking any lines/bytes from an empty file returns None
        assert_eq!(get_start_index(&TakeNum(1), 0), None);

        // Taking more lines/bytes than is available returns None
        assert_eq!(get_start_index(&TakeNum(2), 1), None);

        // When starting line/byte is less than total lines/bytes,
        // return one less than starting number
        assert_eq!(get_start_index(&TakeNum(1), 10), Some(0));
        assert_eq!(get_start_index(&TakeNum(2), 10), Some(1));
        assert_eq!(get_start_index(&TakeNum(3), 10), Some(2));

        // When starting line/byte is negative and less than total,
        // return total - start
        assert_eq!(get_start_index(&TakeNum(-1), 10), Some(9));
        assert_eq!(get_start_index(&TakeNum(-2), 10), Some(8));
        assert_eq!(get_start_index(&TakeNum(-3), 10), Some(7));

        // When starting line/byte is negative and more than total,
        // return 0 to print the whole file
        assert_eq!(get_start_index(&TakeNum(-20), 10), Some(0));
    }
}
