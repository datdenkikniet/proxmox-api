use clap::{Parser, Subcommand};
use proxmox_api::Client;

use access::AccessClient;

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
    pretty_env_logger::init();

    let cli = Cli::parse();

    println!("{cli:?}");

    let (user, realm) = cli
        .user
        .split_once('@')
        .expect("User must be provided as <user>@<realm>");

    let client = Client::new(&cli.host, user, realm, &cli.password).unwrap();

    let client = std::sync::Arc::new(client);

    let access_client = AccessClient::new(client.clone());

    println!("{:?}", access_client.get());
}
