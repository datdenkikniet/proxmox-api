pub mod client;

mod path;
pub use path::{Path, PathElement};

pub mod proxmox_client;
pub use proxmox_client::ProxmoxClient;

pub mod types;

mod generated;
pub use generated::*;

#[cfg(feature = "reqwest-client")]
mod reqwest_client;

#[cfg(feature = "reqwest-client")]
pub use reqwest_client::Client as ReqwestClient;
