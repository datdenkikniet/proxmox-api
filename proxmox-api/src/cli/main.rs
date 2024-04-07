use clap::{Parser, Subcommand};
use proxmox_api::{client::Client, ReqwestClient};

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

    let client = ReqwestClient::new(&cli.host, user, realm, &cli.password).unwrap();

    #[cfg(feature = "nodes")]
    nodes(&client);
}

#[cfg(feature = "nodes")]
fn nodes(client: &impl Client) {
    use proxmox_api::{nodes::NodesClient, types::VmId};

    let nodes_client = NodesClient::new(client);

    println!(
        "VM config: {:#?}",
        nodes_client
            .node("proxmox")
            .qemu()
            .vmid(VmId::new(118).unwrap())
            .config()
            .get(Default::default())
    );
}
