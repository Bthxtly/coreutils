use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello world")
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn omit_newline() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello world")
        .arg("-n")
        .assert()
        .success()
        .stdout("hello world");
}

#[test]
fn lives_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert().success();
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "./tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "./tests/expected/hello2.txt")
}

#[test]
fn hello1_omit_newline() -> TestResult {
    run(&["-n", "Hello  there"], "./tests/expected/hello1.n.txt")
}

#[test]
fn hello2_omit_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "./tests/expected/hello2.n.txt")
}
