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
    about = "Authenticate with password or pub key"
)]
pub enum CliAuthMethod {
    Password {
        #[structopt(short = "p", long = "password")]
        pwd: String,
    },

    PrivateKey {
        #[structopt(short = "k", long = "private_key_path")]
        path: String,
    },
}
