use clap::{Parser, Subcommand};

use crate::client::Client;

mod client;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(env = "PROXMOX_USER")]
    user: String,
    #[clap(env = "PROXMOX_PASSWORD")]
    password: String,
    #[clap(env = "PROXMOX_HOST")]
    host: String,

    #[clap(subcommand)]
    cli_command: CliCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    Login,
}

fn main() {
    let cli = Cli::parse();

    println!("{cli:?}");

    let (user, realm) = cli
        .user
        .split_once('@')
        .expect("User must be provided as <user>@<realm>");

    let mut client = Client::new(&cli.host, user, realm, &cli.password).unwrap();

    println!("{client:?}");

    let status = client.cluster_status();

    println!("{status:#?}");
}
