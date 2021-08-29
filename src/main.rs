use structopt::StructOpt;

use ssh_client::{AuthMethod, Cli, CliAuthMethod, SSHClient};

fn main() {
    let cli: Cli = Cli::from_args();
    let c = match cli.auth_method {
        CliAuthMethod::Password { pwd } => {
            SSHClient::new(cli.address, &cli.username, AuthMethod::Password(&pwd))
        }
        CliAuthMethod::PrivateKey { path } => {
            SSHClient::new(cli.address, &cli.username, AuthMethod::PubKey(&path))
        }
    };

    println!("SSH Response:");
    println!("{}", c.run_command(&cli.command))
}
