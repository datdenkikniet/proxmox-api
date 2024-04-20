pub mod db;
pub mod raw;
pub mod value;
pub struct CfgClient<T> {
    client: T,
    path: String,
}
impl<T> CfgClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/cfg"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a CfgClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> CfgClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &CfgClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> CfgClient<T>
where
    T: crate::client::Client,
{
    pub fn raw(&self) -> raw::RawClient<T> {
        raw::RawClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CfgClient<T>
where
    T: crate::client::Client,
{
    pub fn db(&self) -> db::DbClient<T> {
        db::DbClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CfgClient<T>
where
    T: crate::client::Client,
{
    pub fn value(&self) -> value::ValueClient<T> {
        value::ValueClient::<T>::new(self.client.clone(), &self.path)
    }
}
