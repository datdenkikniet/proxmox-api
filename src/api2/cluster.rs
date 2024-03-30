use serde::Deserialize;

use crate::VmId;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct NextId(pub VmId);

impl NextId {
    pub fn get(&self) -> VmId {
        self.0
    }
}
