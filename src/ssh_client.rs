use std::cell::RefCell;
use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};

use ssh2::Session;

pub struct SSHClient {
    session: RefCell<Session>,
}

impl SSHClient {
    pub fn from_password<A: ToSocketAddrs>(
        socket_address: A,
        username: &str,
        password: &str,
    ) -> anyhow::Result<Self> {
        let tcp = TcpStream::connect(socket_address)?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password(username, password)?;
        Ok(Self {
            session: RefCell::new(session),
        })
    }

    pub fn from_private_key_path<A: ToSocketAddrs>(
        socket_address: A,
        username: &str,
        private_key_path: &str,
    ) -> anyhow::Result<Self> {
        let tcp = TcpStream::connect(socket_address)?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_pubkey_file(username, None, private_key_path.as_ref(), None)?;
        Ok(Self {
            session: RefCell::new(session),
        })
    }
    pub fn run_command(&self, command: &str) -> anyhow::Result<(String, i32)> {
        let mut channel = self.session.borrow_mut().channel_session()?;
        channel.exec(command)?;
        let mut s = String::new();
        channel.read_to_string(&mut s)?;
        channel.close().unwrap();
        let exit_code = channel.exit_status()?;
        Ok((s, exit_code))
    }
}
