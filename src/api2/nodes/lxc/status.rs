use serde::Deserialize;

use crate::{VmId, VmStatus};

#[derive(Debug, Deserialize)]
pub struct Status {
    pub status: VmStatus,
    #[serde(rename = "vmid")]
    pub vm_id: VmId,
    pub tags: Option<String>,
}
