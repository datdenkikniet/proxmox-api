use clap::{Parser, Subcommand};
use proxmox_api::Client;

#[allow(warnings)]
mod generated;

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
    let access_client = generated::access::AccessClient::new(client.clone());

    println!(
        "{:?}",
        access_client.users().userid("root@pam").token().get()
    );

    let cluster_client = generated::cluster::ClusterClient::new(client.clone());

    println!("{:?}", cluster_client.nextid().get(Default::default()));

    let v: Vec<generated::cluster::resources::GETOutputItems> =
        serde_json::from_str(include_str!("./data.json")).unwrap();

    println!(
        "{:?}",
        cluster_client.resources().get(Default::default()).unwrap()[0]
    );
}
