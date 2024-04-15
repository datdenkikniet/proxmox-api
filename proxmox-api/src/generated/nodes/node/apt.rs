pub mod changelog;
pub mod repositories;
pub mod update;
pub mod versions;
pub struct AptClient<T> {
    client: T,
    path: String,
}
impl<T> AptClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/apt"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a AptClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> AptClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index for apt (Advanced Package Tool)."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(id: String) -> Self {
        Self {
            id,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub id: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> AptClient<T>
where
    T: crate::client::Client,
{
    pub fn update(&self) -> update::UpdateClient<T> {
        update::UpdateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AptClient<T>
where
    T: crate::client::Client,
{
    pub fn changelog(&self) -> changelog::ChangelogClient<T> {
        changelog::ChangelogClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AptClient<T>
where
    T: crate::client::Client,
{
    pub fn repositories(&self) -> repositories::RepositoriesClient<T> {
        repositories::RepositoriesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AptClient<T>
where
    T: crate::client::Client,
{
    pub fn versions(&self) -> versions::VersionsClient<T> {
        versions::VersionsClient::<T>::new(self.client.clone(), &self.path)
    }
}
