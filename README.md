# Coreutils in Rust

Implement GNU coreutils in Rust.

This repo is my own learning experience with [Command-Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/)

## Knowledge

### Chapter 1: Truth or Consequences
- Write CLI programs in `/src/bin/<name>` and run them with `cargo run --bin <name>`
- Write tests with `assert_cmd` to test success, failure, and compare stdout

### Chapter 2: Test for Echo
- Process command-line arguments with the `clap` crate
- Test for text that is printed to `STDOUT` and `STDERR`
- Slices and `?` operator

### Chapter 3: On the Catwalk
- Use `Result` to deal with errors
- Separate codes from `main.rs` to `lib.rs`
- Read `stdin` and files with `std::io`
- *test-driven development* (TDD)

### Chapter 4: Head Aches
- Use `BufRead::read_line` instead of `BufRead::lines` to preserve the original line endings
- Use `From::from` to create the `Err` part of `MyResult`

### Chapter 5: Word to Your Mother
- Use `Iterator::all`. There are some similarities like `any`, `filter`, `map`, `find`, `position`,
  `cmp`, `min_by` and `max_by`
- Use `impl trait` as function parameter
- Use `std::io::Cursor` to create a fake file handle

### Chapter 6: Den of Uniquity
- Use `map_err` with `?` to send error information gracefully
- Use closure to reduce repetitive code while capturing values from the enclosing scope
- Use `std::io::Write` trait and `write!()` to output to a file or stdout

### Chapter 7: Finders Keepers
- Use functional style programming(iterator, map, filter)
- Use `walkdir` to find directories and files
- Use `num_args` to accept many values for one option

### Chapter 8: Shave and a Haircut
- Learn more gracefully functional programming(`flatten`, `flat_map`)
- Parse a string with `.parse()` and regular expression(`Regex`)
- Parse `.csv` files with `csv` crate with custom delimiter
- Return `&str` from a function with lifetime indicator

### Chapter 9: Jack the Grepper
- Use `std::mem::take` to take the ownership of the line, which avoids unnecessary copies
- Get file meta data with `std::fs::metadata`
- Iterate lines of a file while preserving newline indicator with `read_line` from `std::io::ReadBuf`

### Chapter 10: Boston Commons
- Use `Ord::cmp` to compare two strings

### Chapter 11: Tailor Swyfte
- Indicate multiple trait bounds on a type using the `where` clause
- Benchmark program with `hyperfine` to compare runtime performance
- Use `std::io::Seek` trait to skip bytes
- Refactor program early and frequently

### Chapter 12: Fortunate Son
- Use `OsStr` and `OsString` to deal with filenames, which makes the project more portable
- Use `Box<dyn trait>` to receive values of different types
- Use `rand` trait to pick elements randomly

### Chapter 13: Rascalry
- Deal with date with `chrono` crate
- Get groups of items in `Vec` with `.chunks(num)`
- Use `itertools::izip` to iterate multiple iterators simultanously
