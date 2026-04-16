pub mod guest;
#[derive(Debug, Clone)]
pub struct BulkActionClient<T> {
    client: T,
    path: String,
}
impl<T> BulkActionClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/bulk-action"),
        }
    }
}
impl<T> BulkActionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List resource types."]
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
impl<T> BulkActionClient<T>
where
    T: crate::client::Client,
{
    pub fn guest(&self) -> guest::GuestClient<T> {
        guest::GuestClient::<T>::new(self.client.clone(), &self.path)
    }
}
