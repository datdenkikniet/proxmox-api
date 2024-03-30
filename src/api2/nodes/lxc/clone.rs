use serde::Serialize;

use crate::{Bool, VmId};

#[derive(Serialize)]
pub struct CloneLxcRequest {
    #[serde(rename = "newid")]
    pub new_id: VmId,

    #[serde(rename = "bwlimit")]
    pub bw_limit: Option<u32>,
    pub description: Option<String>,
    pub full: Option<Bool>,
    pub hostname: Option<String>,
    pub pool: Option<String>,
    pub snapname: Option<String>,
    pub storage: Option<String>,
    #[serde(rename = "target")]
    pub target_node: Option<String>,
}

impl CloneLxcRequest {
    pub fn new(new_id: VmId) -> Self {
        Self {
            new_id,
            bw_limit: None,
            description: None,
            full: None,
            hostname: None,
            pool: None,
            snapname: None,
            storage: None,
            target_node: None,
        }
    }

    pub fn full(mut self, full: bool) -> Self {
        self.full = Some(full.into());
        self
    }

    pub fn pool<T>(mut self, pool: T) -> Self
    where
        T: AsRef<str>,
    {
        self.pool = Some(pool.as_ref().into());
        self
    }

    pub fn hostname<T>(mut self, hostname: T) -> Self
    where
        T: AsRef<str>,
    {
        self.hostname = Some(hostname.as_ref().to_string());
        self
    }
}
