use clap::{Parser, Subcommand};
use proxmox_api::Client;

use proxmox_api::{access::AccessClient, nodes::NodesClient};

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
    println!("{:#?}", access_client.users().get(Default::default()));

    let nodes_client = NodesClient::new(client.clone());
    let lxc = nodes_client.node("proxmox").lxc();
    let nodes = lxc.get().unwrap();

    nodes.iter().for_each(|node| {
        println!("{:?}", lxc.vmid(node.vmid).status().current().get());
    });
}
