use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = cargo_bin_cmd!("signum");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: signum"));
}

#[test]
fn test_cli_remove_chars_integration() {
    let mut cmd = cargo_bin_cmd!("signum");
    let all_possible = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789)-(*&^%$#@!~";
    cmd.arg("100")
        .arg("-r")
        .arg(all_possible)
        .assert()
        .success()
        .stdout(predicate::str::contains("!!!_POOL_EMPTY_!!!"));
}

#[test]
fn test_cli_ambiguous_flag() {
    let mut cmd = cargo_bin_cmd!("signum");

    // Generate 10 passwords of length 50 with the -B flag
    // -B to avoid ambiguous characters (0, 1, l, I)
    cmd.arg("50").arg("10").arg("-B").assert().success().stdout(
        predicate::str::is_match(r"^[a-km-zA-HJ-NP-Z2-9\W\s]+$").unwrap(),
    );
}

#[test]
fn test_cli_no_capitalize_flag() {
    let mut cmd = cargo_bin_cmd!("signum");

    // Generate 20 passwords of length 100 with the -A flag
    cmd.arg("100")
        .arg("20")
        .arg("-A")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[^A-Z]+$").unwrap());
}

#[test]
fn test_cli_digital_only_flag() {
    let mut cmd = cargo_bin_cmd!("signum");

    // Generate 10 passwords of length 20 using the -d flag
    cmd.arg("20")
        .arg("10")
        .arg("-d")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[0-9\s]+$").unwrap());
}

#[test]
fn test_cli_digital_ignores_removal_for_now() {
    let mut cmd = cargo_bin_cmd!("signum");

    // Generate pin and remove '012' from it
    cmd.arg("50")
        .arg("-d")
        .arg("-r")
        .arg("012")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"^[3-9\s]+$").unwrap());
}
