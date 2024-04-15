pub mod cifs;
pub mod glusterfs;
pub mod iscsi;
pub mod lvm;
pub mod lvmthin;
pub mod nfs;
pub mod pbs;
pub mod zfs;
pub struct ScanClient<T> {
    client: T,
    path: String,
}
impl<T> ScanClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/scan"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ScanClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ScanClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Index of available scan methods"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(method: String) -> Self {
        Self {
            method,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub method: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ScanClient<T>
where
    T: crate::client::Client,
{
    pub fn nfs(&self) -> nfs::NfsClient<T> {
        nfs::NfsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ScanClient<T>
where
    T: crate::client::Client,
{
    pub fn cifs(&self) -> cifs::CifsClient<T> {
        cifs::CifsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ScanClient<T>
where
    T: crate::client::Client,
{
    pub fn pbs(&self) -> pbs::PbsClient<T> {
        pbs::PbsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ScanClient<T>
where
    T: crate::client::Client,
{
    pub fn glusterfs(&self) -> glusterfs::GlusterfsClient<T> {
        glusterfs::GlusterfsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ScanClient<T>
where
    T: crate::client::Client,
{
    pub fn iscsi(&self) -> iscsi::IscsiClient<T> {
        iscsi::IscsiClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ScanClient<T>
where
    T: crate::client::Client,
{
    pub fn lvm(&self) -> lvm::LvmClient<T> {
        lvm::LvmClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ScanClient<T>
where
    T: crate::client::Client,
{
    pub fn lvmthin(&self) -> lvmthin::LvmthinClient<T> {
        lvmthin::LvmthinClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ScanClient<T>
where
    T: crate::client::Client,
{
    pub fn zfs(&self) -> zfs::ZfsClient<T> {
        zfs::ZfsClient::<T>::new(self.client.clone(), &self.path)
    }
}
