use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn cli_executes_command_after_password_authentication() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(&[
        "-u",
        "test_user",
        "-c",
        "ls",
        "-a",
        "0.0.0.0:2222",
        "password",
        "test_password",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("SSH Response:\n"));
}

#[test]
fn cli_executes_command_after_private_key_authentication() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(&[
        "-u",
        "test_user",
        "-c",
        "ls",
        "-a",
        "0.0.0.0:2222",
        "private-key",
        "configuration/private_key/id_rsa",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("SSH Response:\n"));
}
