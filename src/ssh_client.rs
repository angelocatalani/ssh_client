use std::cell::RefCell;
use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};

use ssh2::{Channel, Session};

pub struct SSHClient {
    channel: RefCell<Channel>,
}

impl SSHClient {
    pub fn new<A: ToSocketAddrs>(
        socket_address: A,
        username: &str,
        auth_method: AuthMethod,
    ) -> Self {
        let tcp = TcpStream::connect(socket_address).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        match auth_method {
            AuthMethod::Password(password) => {
                sess.userauth_password(&username, &password).unwrap();
                let channel = sess.channel_session().unwrap();
                Self {
                    channel: RefCell::new(channel),
                }
            }
            AuthMethod::PubKey(private_key_path) => {
                sess.userauth_pubkey_file(&username, None, private_key_path.as_ref(), None)
                    .unwrap();
                let channel = sess.channel_session().unwrap();
                Self {
                    channel: RefCell::new(channel),
                }
            }
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

pub enum AuthMethod<'auth> {
    Password(&'auth str),
    PubKey(&'auth str),
}

impl<'auth> AuthMethod<'auth> {
    pub fn from_password(password: &'auth str) -> Self {
        Self::Password(password)
    }
    pub fn from_private_key(private_key_path: &'auth str) -> Self {
        Self::PubKey(private_key_path)
    }
}
