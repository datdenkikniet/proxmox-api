pub struct DumpClient<T> {
    client: T,
    path: String,
}
impl<T> DumpClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/dump"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a DumpClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> DumpClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get automatically generated cloudinit config."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<GetParams, String, T::Error> for &DumpClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: GetParams) -> Result<String, T::Error> {
        self.get(params)
    }
}
impl GetParams {
    pub fn new(ty: Type) -> Self {
        Self {
            ty,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(rename = "type")]
    #[doc = "Config type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Config type."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "meta")]
    Meta,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "user")]
    User,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "meta" => Ok(Self::Meta),
            "network" => Ok(Self::Network),
            "user" => Ok(Self::User),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
