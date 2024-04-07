pub mod cfg;
pub mod cmd_safety;
pub mod crush;
pub mod fs;
pub mod init;
pub mod log;
pub mod mds;
pub mod mgr;
pub mod mon;
pub mod osd;
pub mod pool;
pub mod restart;
pub mod rules;
pub mod start;
pub mod status;
pub mod stop;
pub struct CephClient<T> {
    client: T,
    path: String,
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/ceph"),
        }
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
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
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn cfg(&self) -> cfg::CfgClient<T> {
        cfg::CfgClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn osd(&self) -> osd::OsdClient<T> {
        osd::OsdClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn mds(&self) -> mds::MdsClient<T> {
        mds::MdsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn mgr(&self) -> mgr::MgrClient<T> {
        mgr::MgrClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn mon(&self) -> mon::MonClient<T> {
        mon::MonClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn fs(&self) -> fs::FsClient<T> {
        fs::FsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn pool(&self) -> pool::PoolClient<T> {
        pool::PoolClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn init(&self) -> init::InitClient<T> {
        init::InitClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn stop(&self) -> stop::StopClient<T> {
        stop::StopClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn start(&self) -> start::StartClient<T> {
        start::StartClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn restart(&self) -> restart::RestartClient<T> {
        restart::RestartClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn crush(&self) -> crush::CrushClient<T> {
        crush::CrushClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn log(&self) -> log::LogClient<T> {
        log::LogClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn rules(&self) -> rules::RulesClient<T> {
        rules::RulesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn cmd_safety(&self) -> cmd_safety::CmdSafetyClient<T> {
        cmd_safety::CmdSafetyClient::<T>::new(self.client.clone(), &self.path)
    }
}
