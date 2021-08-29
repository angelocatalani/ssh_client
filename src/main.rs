use ssh_client::{AuthMethod, Cli, CliAuthMethod, SSHClient};
use structopt::StructOpt;
fn main() {
    let cli: Cli = Cli::from_args();
    let c = match cli.auth_method {
        CliAuthMethod::Password { pwd } => {
            SSHClient::new(cli.address, &cli.username, AuthMethod::Password(&pwd))
        }
        CliAuthMethod::PrivateKey { path } => {
            SSHClient::new(cli.address, &cli.username, AuthMethod::Password(&path))
        }
    };

    println!("SSH Response:");
    println!("{}", c.run_command(&cli.command))
}
