use std::io::BufWriter;
use std::io::Write;
use std::thread;
use std::time::Duration;

use anyhow::Context;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use structopt::StructOpt;

use ssh_client::{AuthMethod, Cli, SSHClient};

fn main() -> anyhow::Result<()> {
    match handle_command() {
        Ok(_) => {
            println!("Success");
            std::process::exit(exitcode::OK);
        }
        Err(e) => {
            eprintln!("Error processing command: {}", e);
            std::process::exit(exitcode::USAGE);
        }
    }
}

fn handle_command() -> anyhow::Result<()> {
    let cli: Cli = Cli::from_args();
    let c = match cli.auth_method {
        AuthMethod::Password { pwd } => SSHClient::from_password(cli.address, &cli.username, &pwd)
            .context("Failed to authenticate with username and password")?,
        AuthMethod::PrivateKey { path } => {
            SSHClient::from_private_key_path(cli.address, &cli.username, &path)
                .context("Failed to authenticate with private key")?
        }
    };
    let pb = ProgressBar::new(cli.commands.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("🏝️🦀🌴"),
    );
    let mut stream = BufWriter::new(std::io::stdout());
    cli.commands
        .into_iter()
        .progress_with(pb)
        .map(|command| {
            // sleep to plot a fancy progress bar
            thread::sleep(Duration::from_secs(1));
            let (response, exit_code) = c.run_command(&command).context("Failed to run command")?;
            writeln!(stream, "Command: {}", command).context("Failed to output Command")?;
            writeln!(stream, "Exit Code: {}", exit_code).context("Failed to output Exit Code")?;
            writeln!(stream, "Response: {}", response).context("Failed to output Response")?;
            Ok(())
        })
        .collect()
}
