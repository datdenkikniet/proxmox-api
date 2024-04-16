pub struct LvmthinClient<T> {
    client: T,
    path: String,
}
impl<T> LvmthinClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/lvmthin"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a LvmthinClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> LvmthinClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List local LVM Thin Pools."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<GetParams, Vec<GetOutputItems>, T::Error>
    for &LvmthinClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get(params)
    }
}
impl GetOutputItems {
    pub fn new(lv: String) -> Self {
        Self {
            lv,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The LVM Thin Pool name (LVM logical volume)."]
    #[doc = ""]
    pub lv: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(vg: String) -> Self {
        Self {
            vg,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    pub vg: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
