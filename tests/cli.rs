use assert_cmd::Command;
use predicates::prelude::predicate;

const VALID_COMMANDS: &[&str] = &[
    "-u",
    "test_user",
    "-c",
    "ls",
    "-c",
    "ls",
    "-a",
    "0.0.0.0:2222",
];
const PASSWORD_CREDENTIALS: &[&str] = &["password", "test_password"];
const KEY_CREDENTIALS: &[&str] = &["private-key", "configuration/private_key/id_rsa"];
const INVALID_PASSWORD_CREDENTIALS: &[&str] = &["password", "wrong_password"];

#[test]
fn cli_executes_command_after_password_authentication() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args([VALID_COMMANDS, PASSWORD_CREDENTIALS].concat());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Command: ls\nExit Code: 0\n"));
}

#[test]
fn cli_executes_command_after_private_key_authentication() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args([VALID_COMMANDS, KEY_CREDENTIALS].concat());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Command: ls\nExit Code: 0\n"));
}

#[test]
fn cli_explains_failed_authentication() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args([VALID_COMMANDS, INVALID_PASSWORD_CREDENTIALS].concat());
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error processing command: Failed to authenticate with username and password\n",
    ));
    cmd.assert().code(exitcode::USAGE);
}
