use std::io::Read;
use std::net::TcpStream;

use ssh2::Session;

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
