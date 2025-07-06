pub mod clone;
pub mod config;
pub mod feature;
pub mod firewall;
pub mod interfaces;
pub mod migrate;
pub mod move_volume;
pub mod mtunnel;
pub mod mtunnelwebsocket;
pub mod pending;
pub mod remote_migrate;
pub mod resize;
pub mod rrd;
pub mod rrddata;
pub mod snapshot;
pub mod spiceproxy;
pub mod status;
pub mod template;
pub mod termproxy;
pub mod vncproxy;
pub mod vncwebsocket;
#[derive(Debug, Clone)]
pub struct VmidClient<T> {
    client: T,
    path: String,
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, vmid: crate::types::VmId) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, vmid),
        }
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Destroy the container (also delete all uses files)."]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(rename = "destroy-unreferenced-disks")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If set, destroy additionally all disks with the VMID from all enabled storages which are not referenced in the config."]
    #[doc = ""]
    pub destroy_unreferenced_disks: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Force destroy, even if running."]
    #[doc = ""]
    pub force: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Remove container from all related configurations. For example, backup jobs, replication jobs or HA. Related ACLs and Firewall entries will *always* be removed."]
    #[doc = ""]
    pub purge: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
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
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn config(&self) -> config::ConfigClient<T> {
        config::ConfigClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn snapshot(&self) -> snapshot::SnapshotClient<T> {
        snapshot::SnapshotClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn firewall(&self) -> firewall::FirewallClient<T> {
        firewall::FirewallClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn rrd(&self) -> rrd::RrdClient<T> {
        rrd::RrdClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn rrddata(&self) -> rrddata::RrddataClient<T> {
        rrddata::RrddataClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn vncproxy(&self) -> vncproxy::VncproxyClient<T> {
        vncproxy::VncproxyClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn termproxy(&self) -> termproxy::TermproxyClient<T> {
        termproxy::TermproxyClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn vncwebsocket(&self) -> vncwebsocket::VncwebsocketClient<T> {
        vncwebsocket::VncwebsocketClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn spiceproxy(&self) -> spiceproxy::SpiceproxyClient<T> {
        spiceproxy::SpiceproxyClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn remote_migrate(&self) -> remote_migrate::RemoteMigrateClient<T> {
        remote_migrate::RemoteMigrateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn migrate(&self) -> migrate::MigrateClient<T> {
        migrate::MigrateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn feature(&self) -> feature::FeatureClient<T> {
        feature::FeatureClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn template(&self) -> template::TemplateClient<T> {
        template::TemplateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn clone(&self) -> clone::CloneClient<T> {
        clone::CloneClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn resize(&self) -> resize::ResizeClient<T> {
        resize::ResizeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn move_volume(&self) -> move_volume::MoveVolumeClient<T> {
        move_volume::MoveVolumeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn pending(&self) -> pending::PendingClient<T> {
        pending::PendingClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn interfaces(&self) -> interfaces::InterfacesClient<T> {
        interfaces::InterfacesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn mtunnel(&self) -> mtunnel::MtunnelClient<T> {
        mtunnel::MtunnelClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VmidClient<T>
where
    T: crate::client::Client,
{
    pub fn mtunnelwebsocket(&self) -> mtunnelwebsocket::MtunnelwebsocketClient<T> {
        mtunnelwebsocket::MtunnelwebsocketClient::<T>::new(self.client.clone(), &self.path)
    }
}
