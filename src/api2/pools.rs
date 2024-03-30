use serde::Deserialize;

use crate::{VmId, VmStatus};

#[derive(Debug, Deserialize)]
pub struct PoolData {
    pub members: Vec<PoolMember>,
}

impl PoolData {
    pub fn lxcs(&self) -> impl Iterator<Item = &Lxc> + '_ {
        self.members.iter().filter_map(|v| {
            if let PoolMember::Lxc(lxc) = v {
                Some(lxc)
            } else {
                None
            }
        })
    }

    pub fn qemus(&self) -> impl Iterator<Item = &Qemu> + '_ {
        self.members.iter().filter_map(|v| {
            if let PoolMember::Qemu(qemu) = v {
                Some(qemu)
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum PoolMember {
    Lxc(Lxc),
    Storage(Storage),
    Qemu(Qemu),
}

#[derive(Debug, Deserialize)]
pub struct Lxc {
    #[serde(rename = "vmid")]
    pub vm_id: VmId,
    pub status: VmStatus,
}

#[derive(Debug, Deserialize)]
pub struct Storage {
    pub node: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Qemu {
    #[serde(rename = "vmid")]
    pub vm_id: VmId,
    pub status: VmStatus,
}
