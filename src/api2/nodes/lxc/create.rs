use serde::Serialize;

use crate::Bool;

use super::Config;

#[derive(Debug, Serialize)]
pub struct CreateRequest {
    #[serde(rename = "ostemplate")]
    pub os_template: String,
    #[serde(
        serialize_with = "serialize_keys",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ssh_public_keys: Vec<String>,
    pub start: Option<Bool>,

    pub storage: Option<String>,

    #[serde(flatten)]
    pub config: Config,
}

impl CreateRequest {
    pub fn new(os_template: String) -> Self {
        Self {
            os_template,
            ssh_public_keys: Vec::new(),
            start: None,
            storage: None,
            config: Default::default(),
        }
    }

    pub fn ssh_public_keys(mut self, ssh_public_keys: Vec<String>) -> Self {
        self.ssh_public_keys = ssh_public_keys;
        self
    }

    pub fn start(mut self, start: bool) -> Self {
        self.start = Some(start.into());
        self
    }

    pub fn storage(mut self, storage: String) -> Self {
        self.storage = Some(storage);
        self
    }
}

impl core::ops::Deref for CreateRequest {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl core::ops::DerefMut for CreateRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

fn serialize_keys<S>(keys: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    let mut data = String::new();
    keys.iter().for_each(|k| {
        data.push_str(&k);
        data.push('\n');
    });
    serializer.serialize_str(&data)
}
