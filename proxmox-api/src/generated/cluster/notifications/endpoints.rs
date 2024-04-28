pub mod gotify;
pub mod sendmail;
pub mod smtp;
#[derive(Debug, Clone)]
pub struct EndpointsClient<T> {
    client: T,
    path: String,
}
impl<T> EndpointsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/endpoints"),
        }
    }
}
impl<T> EndpointsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Index for all available endpoint types."]
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
impl<T> EndpointsClient<T>
where
    T: crate::client::Client,
{
    pub fn sendmail(&self) -> sendmail::SendmailClient<T> {
        sendmail::SendmailClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> EndpointsClient<T>
where
    T: crate::client::Client,
{
    pub fn gotify(&self) -> gotify::GotifyClient<T> {
        gotify::GotifyClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> EndpointsClient<T>
where
    T: crate::client::Client,
{
    pub fn smtp(&self) -> smtp::SmtpClient<T> {
        smtp::SmtpClient::<T>::new(self.client.clone(), &self.path)
    }
}
