pub mod aplinfo;
pub mod apt;
pub mod capabilities;
pub mod ceph;
pub mod certificates;
pub mod config;
pub mod disks;
pub mod dns;
pub mod execute;
pub mod firewall;
pub mod hardware;
pub mod hosts;
pub mod journal;
pub mod lxc;
pub mod migrateall;
pub mod netstat;
pub mod network;
pub mod qemu;
pub mod query_url_metadata;
pub mod replication;
pub mod report;
pub mod rrd;
pub mod rrddata;
pub mod scan;
pub mod sdn;
pub mod services;
pub mod spiceshell;
pub mod startall;
pub mod status;
pub mod stopall;
pub mod storage;
pub mod subscription;
pub mod suspendall;
pub mod syslog;
pub mod tasks;
pub mod termproxy;
pub mod time;
pub mod version;
pub mod vncshell;
pub mod vncwebsocket;
pub mod vzdump;
pub mod wakeonlan;
pub struct NodeClient<T> {
    client: T,
    path: String,
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, node: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, node),
        }
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Node index."]
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
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn qemu(&self) -> qemu::QemuClient<T> {
        qemu::QemuClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn lxc(&self) -> lxc::LxcClient<T> {
        lxc::LxcClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn ceph(&self) -> ceph::CephClient<T> {
        ceph::CephClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn vzdump(&self) -> vzdump::VzdumpClient<T> {
        vzdump::VzdumpClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn services(&self) -> services::ServicesClient<T> {
        services::ServicesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn subscription(&self) -> subscription::SubscriptionClient<T> {
        subscription::SubscriptionClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn network(&self) -> network::NetworkClient<T> {
        network::NetworkClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn tasks(&self) -> tasks::TasksClient<T> {
        tasks::TasksClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn scan(&self) -> scan::ScanClient<T> {
        scan::ScanClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn hardware(&self) -> hardware::HardwareClient<T> {
        hardware::HardwareClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn capabilities(&self) -> capabilities::CapabilitiesClient<T> {
        capabilities::CapabilitiesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn storage(&self) -> storage::StorageClient<T> {
        storage::StorageClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn disks(&self) -> disks::DisksClient<T> {
        disks::DisksClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn apt(&self) -> apt::AptClient<T> {
        apt::AptClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn firewall(&self) -> firewall::FirewallClient<T> {
        firewall::FirewallClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn replication(&self) -> replication::ReplicationClient<T> {
        replication::ReplicationClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn certificates(&self) -> certificates::CertificatesClient<T> {
        certificates::CertificatesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn config(&self) -> config::ConfigClient<T> {
        config::ConfigClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn sdn(&self) -> sdn::SdnClient<T> {
        sdn::SdnClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn version(&self) -> version::VersionClient<T> {
        version::VersionClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn netstat(&self) -> netstat::NetstatClient<T> {
        netstat::NetstatClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn execute(&self) -> execute::ExecuteClient<T> {
        execute::ExecuteClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn wakeonlan(&self) -> wakeonlan::WakeonlanClient<T> {
        wakeonlan::WakeonlanClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn rrd(&self) -> rrd::RrdClient<T> {
        rrd::RrdClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn rrddata(&self) -> rrddata::RrddataClient<T> {
        rrddata::RrddataClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn syslog(&self) -> syslog::SyslogClient<T> {
        syslog::SyslogClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn journal(&self) -> journal::JournalClient<T> {
        journal::JournalClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn vncshell(&self) -> vncshell::VncshellClient<T> {
        vncshell::VncshellClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn termproxy(&self) -> termproxy::TermproxyClient<T> {
        termproxy::TermproxyClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn vncwebsocket(&self) -> vncwebsocket::VncwebsocketClient<T> {
        vncwebsocket::VncwebsocketClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn spiceshell(&self) -> spiceshell::SpiceshellClient<T> {
        spiceshell::SpiceshellClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn dns(&self) -> dns::DnsClient<T> {
        dns::DnsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn time(&self) -> time::TimeClient<T> {
        time::TimeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn aplinfo(&self) -> aplinfo::AplinfoClient<T> {
        aplinfo::AplinfoClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn query_url_metadata(&self) -> query_url_metadata::QueryUrlMetadataClient<T> {
        query_url_metadata::QueryUrlMetadataClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn report(&self) -> report::ReportClient<T> {
        report::ReportClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn startall(&self) -> startall::StartallClient<T> {
        startall::StartallClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn stopall(&self) -> stopall::StopallClient<T> {
        stopall::StopallClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn suspendall(&self) -> suspendall::SuspendallClient<T> {
        suspendall::SuspendallClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn migrateall(&self) -> migrateall::MigrateallClient<T> {
        migrateall::MigrateallClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn hosts(&self) -> hosts::HostsClient<T> {
        hosts::HostsClient::<T>::new(self.client.clone(), &self.path)
    }
}
