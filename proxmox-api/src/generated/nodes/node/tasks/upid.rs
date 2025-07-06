pub mod log;
pub mod status;
#[derive(Debug, Clone)]
pub struct UpidClient<T> {
    client: T,
    path: String,
}
impl<T> UpidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, upid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, upid),
        }
    }
}
impl<T> UpidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Stop a task."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> UpidClient<T>
where
    T: crate::client::Client,
{
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
impl<T> UpidClient<T>
where
    T: crate::client::Client,
{
    pub fn log(&self) -> log::LogClient<T> {
        log::LogClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> UpidClient<T>
where
    T: crate::client::Client,
{
    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
