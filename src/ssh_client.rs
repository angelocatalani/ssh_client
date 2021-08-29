use std::cell::RefCell;
use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};

use ssh2::{Channel, Session};

pub struct SSHClient {
    channel: RefCell<Channel>,
}

impl SSHClient {
    pub fn from_password<A: ToSocketAddrs>(
        socket_address: A,
        username: &str,
        password: &str,
    ) -> Self {
        let tcp = TcpStream::connect(socket_address).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_password(&username, &password).unwrap();
        let channel = sess.channel_session().unwrap();
        Self {
            channel: RefCell::new(channel),
        }
    }

    pub fn from_private_key_path<A: ToSocketAddrs>(
        socket_address: A,
        username: &str,
        private_key_path: &str,
    ) -> Self {
        let tcp = TcpStream::connect(socket_address).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_pubkey_file(&username, None, private_key_path.as_ref(), None)
            .unwrap();
        let channel = sess.channel_session().unwrap();
        Self {
            channel: RefCell::new(channel),
        }
    }
    pub fn run_command(&self, command: &str) -> String {
        self.channel.borrow_mut().exec(command).unwrap();
        let mut s = String::new();
        self.channel.borrow_mut().read_to_string(&mut s).unwrap();
        s
    }
}

impl Drop for SSHClient {
    fn drop(&mut self) {
        self.channel.borrow_mut().close().unwrap()
    }
}
