pub struct FlagClient<T> {
    client: T,
    path: String,
}
impl<T> FlagClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, flag: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, flag),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a FlagClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> FlagClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the status of a specific ceph flag."]
    #[doc = ""]
    pub fn get(&self) -> Result<bool, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        Ok(self.client.get::<_, crate::types::Bool>(&path, &())?.get())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), bool, T::Error> for &FlagClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<bool, T::Error> {
        self.get()
    }
}
impl<T> FlagClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set or clear (unset) a specific ceph flag"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PutParams, (), T::Error> for &FlagClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Put;
    fn exec(&self, params: PutParams) -> Result<(), T::Error> {
        self.put(params)
    }
}
impl PutParams {
    pub fn new(value: bool) -> Self {
        Self {
            value,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "The new value of the flag"]
    #[doc = ""]
    pub value: bool,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
