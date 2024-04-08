pub mod sid;
pub struct ResourcesClient<T> {
    client: T,
    path: String,
}
impl<T> ResourcesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/resources"),
        }
    }
}
impl<T> ResourcesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List HA resources."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> ResourcesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new HA resource."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(sid: String) -> Self {
        Self {
            sid,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub sid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list resources of specific type"]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(sid: String) -> Self {
        Self {
            sid,
            comment: Default::default(),
            group: Default::default(),
            max_relocate: Default::default(),
            max_restart: Default::default(),
            state: Default::default(),
            ty: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description."]
    #[doc = ""]
    pub comment: Option<String>,
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
    pub state: Option<State>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Resource type."]
    #[doc = ""]
    pub ty: Option<Type>,
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
impl Default for State {
    fn default() -> Self {
        Self::Started
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Only list resources of specific type"]
#[doc = ""]
pub enum Type {
    #[serde(rename = "ct")]
    Ct,
    #[serde(rename = "vm")]
    Vm,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ct" => Ok(Self::Ct),
            "vm" => Ok(Self::Vm),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> ResourcesClient<T>
where
    T: crate::client::Client,
{
    pub fn sid(&self, sid: &str) -> sid::SidClient<T> {
        sid::SidClient::<T>::new(self.client.clone(), &self.path, sid)
    }
}
