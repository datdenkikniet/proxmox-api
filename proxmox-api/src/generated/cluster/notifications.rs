pub mod endpoints;
pub mod matchers;
pub mod targets;
pub struct NotificationsClient<T> {
    client: T,
    path: String,
}
impl<T> NotificationsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/notifications"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a NotificationsClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> NotificationsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Index for notification-related API endpoints."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
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
impl<T> NotificationsClient<T>
where
    T: crate::client::Client,
{
    pub fn endpoints(&self) -> endpoints::EndpointsClient<T> {
        endpoints::EndpointsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NotificationsClient<T>
where
    T: crate::client::Client,
{
    pub fn targets(&self) -> targets::TargetsClient<T> {
        targets::TargetsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NotificationsClient<T>
where
    T: crate::client::Client,
{
    pub fn matchers(&self) -> matchers::MatchersClient<T> {
        matchers::MatchersClient::<T>::new(self.client.clone(), &self.path)
    }
}
