pub struct NotBackedUpClient<T> {
    client: T,
    path: String,
}
impl<T> NotBackedUpClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/not-backed-up"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "lxc")]
    Lxc,
    #[serde(rename = "qemu")]
    Qemu,
}
impl GetOutputItems {
    pub fn new(ty: Type, vmid: u64) -> Self {
        Self {
            ty,
            vmid,
            name: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Name of the guest"]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Type of the guest."]
    pub ty: Type,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    #[doc = "VMID of the guest."]
    pub vmid: u64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> NotBackedUpClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Shows all guests which are not covered by any backup job."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
