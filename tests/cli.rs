use assert_cmd::Command;
use pretty_assertions::assert_eq;
use std::fs;

const PRG: &str = "asciir";
const EXPECTED_FILE: &str = "./tests/expected/table.txt";

// --------------------------------------------------
#[test]
fn runs() {
    let expected = fs::read_to_string(EXPECTED_FILE).unwrap();
    let output = Command::cargo_bin(PRG).unwrap().output().expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
}
