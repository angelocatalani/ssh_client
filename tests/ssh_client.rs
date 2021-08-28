use std::io::Read;
use std::net::TcpStream;

use ssh2::Session;

#[test]
fn connect_to_ssh_server_with_username_password() {
    let tcp = TcpStream::connect("0.0.0.0:2222").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("test_user", "test_password")
        .unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("<{}>", s);
    channel.wait_close().ok();
    println!("<{}>", channel.exit_status().unwrap());
}

#[test]
fn connect_to_ssh_server_with_private_key() {
    let tcp = TcpStream::connect("0.0.0.0:2222").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(
        "test_user",
        None,
        "configuration/private_key/id_rsa".as_ref(),
        None,
    )
    .unwrap();
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("<{}>", s);
    channel.wait_close().ok();
    println!("<{}>", channel.exit_status().unwrap());
}
