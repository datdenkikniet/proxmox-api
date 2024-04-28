pub mod acme;
pub mod backup;
pub mod backup_info;
pub mod ceph;
pub mod config;
pub mod firewall;
pub mod ha;
pub mod jobs;
pub mod log;
pub mod mapping;
pub mod metrics;
pub mod nextid;
pub mod notifications;
pub mod options;
pub mod replication;
pub mod resources;
pub mod sdn;
pub mod status;
pub mod tasks;
#[derive(Debug, Clone)]
pub struct ClusterClient<T> {
    client: T,
    path: String,
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "/cluster".to_string(),
        }
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Cluster index."]
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
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn replication(&self) -> replication::ReplicationClient<T> {
        replication::ReplicationClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn metrics(&self) -> metrics::MetricsClient<T> {
        metrics::MetricsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn notifications(&self) -> notifications::NotificationsClient<T> {
        notifications::NotificationsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn config(&self) -> config::ConfigClient<T> {
        config::ConfigClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn firewall(&self) -> firewall::FirewallClient<T> {
        firewall::FirewallClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn backup(&self) -> backup::BackupClient<T> {
        backup::BackupClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn backup_info(&self) -> backup_info::BackupInfoClient<T> {
        backup_info::BackupInfoClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn ha(&self) -> ha::HaClient<T> {
        ha::HaClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn acme(&self) -> acme::AcmeClient<T> {
        acme::AcmeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn ceph(&self) -> ceph::CephClient<T> {
        ceph::CephClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn jobs(&self) -> jobs::JobsClient<T> {
        jobs::JobsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn mapping(&self) -> mapping::MappingClient<T> {
        mapping::MappingClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn sdn(&self) -> sdn::SdnClient<T> {
        sdn::SdnClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn log(&self) -> log::LogClient<T> {
        log::LogClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn resources(&self) -> resources::ResourcesClient<T> {
        resources::ResourcesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn tasks(&self) -> tasks::TasksClient<T> {
        tasks::TasksClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn options(&self) -> options::OptionsClient<T> {
        options::OptionsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ClusterClient<T>
where
    T: crate::client::Client,
{
    pub fn nextid(&self) -> nextid::NextidClient<T> {
        nextid::NextidClient::<T>::new(self.client.clone(), &self.path)
    }
}
