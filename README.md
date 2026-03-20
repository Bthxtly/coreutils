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
