use clap::Parser;
use proxmox_api::{client::Client, ReqwestClient};

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(env = "PROXMOX_HOST")]
    host: String,

    /// <user>@<realm>
    #[clap(env = "PROXMOX_USER")]
    user: String,

    /// The password to use.
    #[clap(long, env = "PROXMOX_PASSWORD")]
    password: Option<String>,

    /// The API token to use.
    #[clap(long, env = "PROXMOX_API_TOKEN")]
    token: Option<String>,

    /// The API token ID. Required if `token` is set.
    #[clap(long, env = "PROXMOX_API_TOKEN_ID")]
    token_id: Option<String>,
}

impl Cli {
    pub fn client(&self) -> std::io::Result<impl Client + std::fmt::Debug> {
        fn err<T: std::fmt::Display>(t: T) -> std::io::Error {
            std::io::Error::other(format!("{t}"))
        }

        fn err_dbg<T: std::fmt::Debug>(t: T) -> std::io::Error {
            std::io::Error::other(format!("Error while creating client: {t:?}"))
        }

        let (user, realm) = self
            .user
            .split_once('@')
            .expect("User must be provided as <user>@<realm>");

        if let Some(token) = &self.token {
            let token_id = self.token_id.as_ref().ok_or(err("API token requires ID"))?;

            ReqwestClient::new_with_api_token(&self.host, user, realm, token_id, token)
                .map_err(err_dbg)
        } else {
            let password = self
                .password
                .as_ref()
                .expect("Password or token is required.");

            ReqwestClient::new(&self.host, &user, &realm, &password).map_err(err_dbg)
        }
    }
}

fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let cli = Cli::parse();

    println!("{cli:?}");

    let client = cli.client()?;

    #[cfg(feature = "nodes")]
    nodes(&client);

    #[cfg(feature = "pools")]
    pools(&client);

    Ok(())
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

#[cfg(feature = "pools")]
fn pools(client: &impl Client) {
    use proxmox_api::pools::{GetParams, PoolsClient};

    let pools_client = PoolsClient::new(client);

    pools_client
        .get(Default::default())
        .unwrap()
        .into_iter()
        .for_each(|v| {
            let pool = pools_client
                .get(GetParams {
                    poolid: Some(v.poolid.clone()),
                    ..Default::default()
                })
                .unwrap();

            println!("Pool info for {}: {:?}", v.poolid, pool[0]);
        });
}
