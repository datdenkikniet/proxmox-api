use clap::{Parser, Subcommand};
use proxmox_api::{client::Client, nodes::lxc::CloneLxcRequest, VmId};

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
    let proxmox = client.node("proxmox");

    let status = client.cluster_status();
    println!("Status: {status:#?}");

    let next_id = client.cluster_nextid().unwrap();
    println!("nextid: {next_id:?}");

    let req = CloneLxcRequest::new(next_id.get())
        .full(false)
        .pool("pool")
        .pool("github-runners")
        .hostname(format!("runner-{}", next_id.get()));

    // let clone_result = proxmox.lxc(VmId::new(125).unwrap()).clone(&req);
    // println!("{clone_result:?}");

    let pool_info = client.pool_info("github-runners");
    println!("{pool_info:?}");

    let id = VmId::new(105).unwrap();
    let lxc = proxmox.lxc(id);
    println!("{id} status: {:?}", lxc.status());

    let interfaces = lxc.interfaces();
    println!("{interfaces:?}");

    println!("{:?}", proxmox.hosts());
}
