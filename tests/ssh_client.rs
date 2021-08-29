use assert_cmd::Command;
use predicates::prelude::predicate;
use ssh_client::{AuthMethod, SSHClient};

#[test]
fn ssh_client_connects_to_ssh_server_with_username_password() {
    let c = SSHClient::new(
        "0.0.0.0:2222",
        "test_user",
        AuthMethod::from_password("test_password"),
    );
    c.run_command("ls");
}

#[test]
fn ssh_client_connects_to_ssh_server_with_private_key() {
    let c = SSHClient::new(
        "0.0.0.0:2222",
        "test_user",
        AuthMethod::from_private_key("configuration/private_key/id_rsa"),
    );
    c.run_command("ls");
}

#[test]
fn cli_executes_command_with_password_authentication() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(&[
        "-u",
        "test_user",
        "-c",
        "ls",
        "-a",
        "0.0.0.0:2222",
        "password",
        "--password",
        "test_password",
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("SSH Response:\n"));
}
