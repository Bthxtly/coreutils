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
