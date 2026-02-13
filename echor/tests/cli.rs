
use assert_cmd::Command;
use predicates::prelude::*;

// To test against the expected folder's files, we use fs module
use std::fs;


// Using the .unwrap() assumes a fallible path will never be reached, which might cause panics, so wrap custom type
type TestResult = Result<(), Box<dyn std::error::Error>>; 

#[test]
fn dies_no_args() -> TestResult {
    // Get the binary
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}


#[test]
fn run() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello")
        .assert()
        .success();

    Ok(())
}

#[test]
fn test_hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected_output = fs::read_to_string(outfile)?;

    // Let's start asserting via the assert_cmd
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there")
        .assert()
        .success()
        .stdout(expected_output);

    Ok(())
}

#[test]
fn test_hello2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    let expected_output = fs::read_to_string(outfile)?;

    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there")
        .assert()
        .success()
        .stdout(expected_output);

    Ok(())
}

// Test for the flags
#[test]
fn hello1n() -> TestResult {

    let args: &[&str] = &["-n", "Hello  there"];
    let outfile = "tests/expected/hello1.n.txt";
    let expected_output = fs::read_to_string(outfile)?;

    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args)
    .assert()
    .success()
    .stdout(expected_output);

    Ok(())
}


#[test]
fn test_hello2n() -> TestResult {

    let args: &[&str] = &["Hello", "there", "-n"];
    let outfile = "tests/expected/hello2.n.txt";
    let expected_output = fs::read_to_string(outfile)?;

    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args)
    .assert()
    .success()
    .stdout(expected_output);
    Ok(())
}


