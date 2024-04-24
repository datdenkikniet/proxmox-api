pub mod account;
pub mod challenge_schema;
pub mod directories;
pub mod meta;
pub mod plugins;
pub mod tos;
#[derive(Debug, Clone)]
pub struct AcmeClient<T> {
    client: T,
    path: String,
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/acme"),
        }
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "ACMEAccount index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
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
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn plugins(&self) -> plugins::PluginsClient<T> {
        plugins::PluginsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn account(&self) -> account::AccountClient<T> {
        account::AccountClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn tos(&self) -> tos::TosClient<T> {
        tos::TosClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn meta(&self) -> meta::MetaClient<T> {
        meta::MetaClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn directories(&self) -> directories::DirectoriesClient<T> {
        directories::DirectoriesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AcmeClient<T>
where
    T: crate::client::Client,
{
    pub fn challenge_schema(&self) -> challenge_schema::ChallengeSchemaClient<T> {
        challenge_schema::ChallengeSchemaClient::<T>::new(self.client.clone(), &self.path)
    }
}
