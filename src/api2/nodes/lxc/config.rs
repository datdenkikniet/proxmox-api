use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};

use crate::{Bool, PveVolumeId};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "onboot")]
    pub start_on_boot: Option<Bool>,

    /// The amount of memory in Megabytes.
    #[serde(rename = "memory")]
    pub memory_mb: Option<u32>,

    pub cores: Option<NonZeroU16>,

    pub hostname: Option<String>,

    pub hookscripts: Option<PveVolumeId>,
}

impl Config {
    pub fn start_on_boot(mut self, start_on_boot: bool) -> Self {
        self.start_on_boot = Some(start_on_boot.into());
        self
    }

    pub fn memory_mb(mut self, memory_mb: u32) -> Self {
        self.memory_mb = Some(memory_mb.max(512));
        self
    }

    pub fn cores(mut self, cores: NonZeroU16) -> Self {
        self.cores = Some(cores);
        self
    }

    pub fn hostname<T>(mut self, name: T) -> Self
    where
        T: Into<String>,
    {
        self.hostname = Some(name.into());
        self
    }
}
