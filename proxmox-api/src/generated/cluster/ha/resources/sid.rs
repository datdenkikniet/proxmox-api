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
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete resource configuration."]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read resource configuration."]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update resource configuration."]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
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
    pub comment: Option<String>,
    #[doc = "Can be used to prevent concurrent modifications."]
    pub digest: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The HA group identifier."]
    pub group: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of service relocate tries when a service failes to start."]
    pub max_relocate: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of tries to restart the service on a node after its start failed."]
    pub max_restart: Option<u64>,
    #[doc = "HA resource ID. This consists of a resource type followed by a resource specific name, separated with colon (example: vm:100 / ct:100). For virtual machines and containers, you can simply use the VM or CT id as a shortcut (example: 100)."]
    pub sid: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Requested resource state."]
    pub state: Option<State>,
    #[serde(rename = "type")]
    #[doc = "The type of the resources."]
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
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The HA group identifier."]
    pub group: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of service relocate tries when a service failes to start."]
    pub max_relocate: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of tries to restart the service on a node after its start failed."]
    pub max_restart: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Requested resource state."]
    #[doc = "Requested resource state. The CRM reads this state and acts accordingly.\nPlease note that `enabled` is just an alias for `started`.\n\n`started`;;\n\nThe CRM tries to start the resource. Service state is\nset to `started` after successful start. On node failures, or when start\nfails, it tries to recover the resource.  If everything fails, service\nstate it set to `error`.\n\n`stopped`;;\n\nThe CRM tries to keep the resource in `stopped` state, but it\nstill tries to relocate the resources on node failures.\n\n`disabled`;;\n\nThe CRM tries to put the resource in `stopped` state, but does not try\nto relocate the resources on node failures. The main purpose of this\nstate is error recovery, because it is the only way to move a resource out\nof the `error` state.\n\n`ignored`;;\n\nThe resource gets removed from the manager status and so the CRM and the LRM do\nnot touch the resource anymore. All {pve} API calls affecting this resource\nwill be executed, directly bypassing the HA stack. CRM commands will be thrown\naway while there source is in this state. The resource will not get relocated\non node failures.\n\n"]
    pub state: Option<State>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
