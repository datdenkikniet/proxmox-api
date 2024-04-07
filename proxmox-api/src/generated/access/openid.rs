pub mod auth_url;
pub mod login;
pub struct OpenidClient<T> {
    client: T,
    path: String,
}
impl<T> OpenidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/openid"),
        }
    }
}
impl<T> OpenidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(subdir: String) -> Self {
        Self {
            subdir,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub subdir: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> OpenidClient<T>
where
    T: crate::client::Client,
{
    pub fn auth_url(&self) -> auth_url::AuthUrlClient<T> {
        auth_url::AuthUrlClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> OpenidClient<T>
where
    T: crate::client::Client,
{
    pub fn login(&self) -> login::LoginClient<T> {
        login::LoginClient::<T>::new(self.client.clone(), &self.path)
    }
}
