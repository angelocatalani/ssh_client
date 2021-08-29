use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "SSH Client", about = "Run commands using the SSH Rust Client")]
pub struct Cli {
    #[structopt(short = "u", long = "username")]
    pub username: String,
    #[structopt(subcommand)]
    pub auth_method: CliAuthMethod,
    #[structopt(short = "c", long = "command")]
    pub command: String,
    #[structopt(short = "a", long = "address")]
    pub address: String,
}

#[derive(StructOpt)]
#[structopt(
    name = "Authentication method",
    about = "Authenticate with password or private key file path"
)]
pub enum CliAuthMethod {
    #[structopt(about = "The password to authenticate the user")]
    Password { pwd: String },
    #[structopt(
        about = "The private key file path, associated to the public key known to the SSH server"
    )]
    PrivateKey { path: String },
}
