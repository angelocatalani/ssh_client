use structopt::StructOpt;

use ssh_client::{AuthMethod, Cli, SSHClient};

fn main() {
    let cli: Cli = Cli::from_args();
    let c = match cli.auth_method {
        AuthMethod::Password { pwd } => SSHClient::from_password(cli.address, &cli.username, &pwd),
        AuthMethod::PrivateKey { path } => {
            SSHClient::from_private_key_path(cli.address, &cli.username, &path)
        }
    };

    println!("SSH Response:");
    println!("{}", c.run_command(&cli.command))
}
