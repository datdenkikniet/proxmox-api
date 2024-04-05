pub mod config;
pub mod rollback;
pub struct SnapnameClient<T> {
    client: T,
    path: String,
}
impl<T> SnapnameClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, snapname: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, snapname),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For removal from config file, even if removing disk snapshots fails."]
    pub force: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> SnapnameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete a LXC snapshot."]
    pub fn delete(&self, params: DeleteParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
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
impl<T> SnapnameClient<T>
where
    T: crate::client::Client,
{
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> SnapnameClient<T>
where
    T: crate::client::Client,
{
    pub fn rollback(&self) -> rollback::RollbackClient<T> {
        rollback::RollbackClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SnapnameClient<T>
where
    T: crate::client::Client,
{
    pub fn config(&self) -> config::ConfigClient<T> {
        config::ConfigClient::<T>::new(self.client.clone(), &self.path)
    }
}
