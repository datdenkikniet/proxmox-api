pub mod migrate;
pub mod relocate;
pub struct SidClient<T> {
    client: T,
    path: String,
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, sid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, sid),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a SidClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete resource configuration."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.delete(&path, &())
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read resource configuration."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update resource configuration."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
    }
}
impl GetOutput {
    pub fn new(digest: String, sid: String, ty: String) -> Self {
        Self {
            digest,
            sid,
            ty,
            comment: Default::default(),
            group: Default::default(),
            max_relocate: Default::default(),
            max_restart: Default::default(),
            state: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description."]
    #[doc = ""]
    pub comment: Option<String>,
    #[doc = "Can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The HA group identifier."]
    #[doc = ""]
    pub group: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of service relocate tries when a service failes to start."]
    #[doc = ""]
    pub max_relocate: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of tries to restart the service on a node after its start failed."]
    #[doc = ""]
    pub max_restart: Option<u64>,
    #[doc = "HA resource ID. This consists of a resource type followed by a resource specific name, separated with colon (example: vm:100 / ct:100). For virtual machines and containers, you can simply use the VM or CT id as a shortcut (example: 100)."]
    #[doc = ""]
    pub sid: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Requested resource state."]
    #[doc = ""]
    pub state: Option<State>,
    #[serde(rename = "type")]
    #[doc = "The type of the resources."]
    #[doc = ""]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description."]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The HA group identifier."]
    #[doc = ""]
    pub group: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of service relocate tries when a service failes to start."]
    #[doc = ""]
    pub max_relocate: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of tries to restart the service on a node after its start failed."]
    #[doc = ""]
    pub max_restart: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Requested resource state."]
    #[doc = ""]
    #[doc = "Requested resource state. The CRM reads this state and acts accordingly."]
    #[doc = ""]
    #[doc = "Please note that `enabled` is just an alias for `started`."]
    #[doc = ""]
    #[doc = "`started`;;"]
    #[doc = ""]
    #[doc = "The CRM tries to start the resource. Service state is"]
    #[doc = ""]
    #[doc = "set to `started` after successful start. On node failures, or when start"]
    #[doc = ""]
    #[doc = "fails, it tries to recover the resource.  If everything fails, service"]
    #[doc = ""]
    #[doc = "state it set to `error`."]
    #[doc = ""]
    #[doc = "`stopped`;;"]
    #[doc = ""]
    #[doc = "The CRM tries to keep the resource in `stopped` state, but it"]
    #[doc = ""]
    #[doc = "still tries to relocate the resources on node failures."]
    #[doc = ""]
    #[doc = "`disabled`;;"]
    #[doc = ""]
    #[doc = "The CRM tries to put the resource in `stopped` state, but does not try"]
    #[doc = ""]
    #[doc = "to relocate the resources on node failures. The main purpose of this"]
    #[doc = ""]
    #[doc = "state is error recovery, because it is the only way to move a resource out"]
    #[doc = ""]
    #[doc = "of the `error` state."]
    #[doc = ""]
    #[doc = "`ignored`;;"]
    #[doc = ""]
    #[doc = "The resource gets removed from the manager status and so the CRM and the LRM do"]
    #[doc = ""]
    #[doc = "not touch the resource anymore. All {pve} API calls affecting this resource"]
    #[doc = ""]
    #[doc = "will be executed, directly bypassing the HA stack. CRM commands will be thrown"]
    #[doc = ""]
    #[doc = "away while there source is in this state. The resource will not get relocated"]
    #[doc = ""]
    #[doc = "on node failures."]
    #[doc = ""]
    pub state: Option<State2>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Requested resource state."]
#[doc = ""]
pub enum State {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "stopped")]
    Stopped,
}
impl TryFrom<&str> for State {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "disabled" => Ok(Self::Disabled),
            "enabled" => Ok(Self::Enabled),
            "ignored" => Ok(Self::Ignored),
            "started" => Ok(Self::Started),
            "stopped" => Ok(Self::Stopped),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Requested resource state."]
#[doc = ""]
#[doc = "Requested resource state. The CRM reads this state and acts accordingly."]
#[doc = ""]
#[doc = "Please note that `enabled` is just an alias for `started`."]
#[doc = ""]
#[doc = "`started`;;"]
#[doc = ""]
#[doc = "The CRM tries to start the resource. Service state is"]
#[doc = ""]
#[doc = "set to `started` after successful start. On node failures, or when start"]
#[doc = ""]
#[doc = "fails, it tries to recover the resource.  If everything fails, service"]
#[doc = ""]
#[doc = "state it set to `error`."]
#[doc = ""]
#[doc = "`stopped`;;"]
#[doc = ""]
#[doc = "The CRM tries to keep the resource in `stopped` state, but it"]
#[doc = ""]
#[doc = "still tries to relocate the resources on node failures."]
#[doc = ""]
#[doc = "`disabled`;;"]
#[doc = ""]
#[doc = "The CRM tries to put the resource in `stopped` state, but does not try"]
#[doc = ""]
#[doc = "to relocate the resources on node failures. The main purpose of this"]
#[doc = ""]
#[doc = "state is error recovery, because it is the only way to move a resource out"]
#[doc = ""]
#[doc = "of the `error` state."]
#[doc = ""]
#[doc = "`ignored`;;"]
#[doc = ""]
#[doc = "The resource gets removed from the manager status and so the CRM and the LRM do"]
#[doc = ""]
#[doc = "not touch the resource anymore. All {pve} API calls affecting this resource"]
#[doc = ""]
#[doc = "will be executed, directly bypassing the HA stack. CRM commands will be thrown"]
#[doc = ""]
#[doc = "away while there source is in this state. The resource will not get relocated"]
#[doc = ""]
#[doc = "on node failures."]
#[doc = ""]
pub enum State2 {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "stopped")]
    Stopped,
}
impl TryFrom<&str> for State2 {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "disabled" => Ok(Self::Disabled),
            "enabled" => Ok(Self::Enabled),
            "ignored" => Ok(Self::Ignored),
            "started" => Ok(Self::Started),
            "stopped" => Ok(Self::Stopped),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for State2 {
    fn default() -> Self {
        Self::Started
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    pub fn migrate(&self) -> migrate::MigrateClient<T> {
        migrate::MigrateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    pub fn relocate(&self) -> relocate::RelocateClient<T> {
        relocate::RelocateClient::<T>::new(self.client.clone(), &self.path)
    }
}
