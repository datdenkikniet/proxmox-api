pub mod directory;
pub mod initgpt;
pub mod list;
pub mod lvm;
pub mod lvmthin;
pub mod smart;
pub mod wipedisk;
pub mod zfs;
pub struct DisksClient<T> {
    client: T,
    path: String,
}
impl<T> DisksClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/disks"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a DisksClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> DisksClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Node index."]
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
impl<T> DisksClient<T>
where
    T: crate::client::Client,
{
    pub fn lvm(&self) -> lvm::LvmClient<T> {
        lvm::LvmClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> DisksClient<T>
where
    T: crate::client::Client,
{
    pub fn lvmthin(&self) -> lvmthin::LvmthinClient<T> {
        lvmthin::LvmthinClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> DisksClient<T>
where
    T: crate::client::Client,
{
    pub fn directory(&self) -> directory::DirectoryClient<T> {
        directory::DirectoryClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> DisksClient<T>
where
    T: crate::client::Client,
{
    pub fn zfs(&self) -> zfs::ZfsClient<T> {
        zfs::ZfsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> DisksClient<T>
where
    T: crate::client::Client,
{
    pub fn list(&self) -> list::ListClient<T> {
        list::ListClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> DisksClient<T>
where
    T: crate::client::Client,
{
    pub fn smart(&self) -> smart::SmartClient<T> {
        smart::SmartClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> DisksClient<T>
where
    T: crate::client::Client,
{
    pub fn initgpt(&self) -> initgpt::InitgptClient<T> {
        initgpt::InitgptClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> DisksClient<T>
where
    T: crate::client::Client,
{
    pub fn wipedisk(&self) -> wipedisk::WipediskClient<T> {
        wipedisk::WipediskClient::<T>::new(self.client.clone(), &self.path)
    }
}
