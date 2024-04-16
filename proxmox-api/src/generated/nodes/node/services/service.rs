pub mod reload;
pub mod restart;
pub mod start;
pub mod state;
pub mod stop;
pub struct ServiceClient<T> {
    client: T,
    path: String,
}
impl<T> ServiceClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, service: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, service),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ServiceClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ServiceClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &ServiceClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
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
impl<T> ServiceClient<T>
where
    T: crate::client::Client,
{
    pub fn state(&self) -> state::StateClient<T> {
        state::StateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ServiceClient<T>
where
    T: crate::client::Client,
{
    pub fn start(&self) -> start::StartClient<T> {
        start::StartClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ServiceClient<T>
where
    T: crate::client::Client,
{
    pub fn stop(&self) -> stop::StopClient<T> {
        stop::StopClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ServiceClient<T>
where
    T: crate::client::Client,
{
    pub fn restart(&self) -> restart::RestartClient<T> {
        restart::RestartClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ServiceClient<T>
where
    T: crate::client::Client,
{
    pub fn reload(&self) -> reload::ReloadClient<T> {
        reload::ReloadClient::<T>::new(self.client.clone(), &self.path)
    }
}
