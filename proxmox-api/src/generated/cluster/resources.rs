pub struct ResourcesClient<T> {
    client: T,
    path: String,
}
impl<T> ResourcesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/resources"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "lxc")]
    Lxc,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "openvz")]
    Openvz,
    #[serde(rename = "pool")]
    Pool,
    #[serde(rename = "qemu")]
    Qemu,
    #[serde(rename = "sdn")]
    Sdn,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "vm")]
    Vm,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(id: String, ty: Type) -> Self {
        Self {
            id,
            ty,
            cgroup_mode: Default::default(),
            content: Default::default(),
            cpu: Default::default(),
            disk: Default::default(),
            hastate: Default::default(),
            level: Default::default(),
            maxcpu: Default::default(),
            maxdisk: Default::default(),
            maxmem: Default::default(),
            mem: Default::default(),
            name: Default::default(),
            node: Default::default(),
            plugintype: Default::default(),
            pool: Default::default(),
            status: Default::default(),
            storage: Default::default(),
            uptime: Default::default(),
            vmid: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(rename = "cgroup-mode")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The cgroup mode the node operates under (when type == node)."]
    pub cgroup_mode: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allowed storage content types (when type == storage)."]
    pub content: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CPU utilization (when type in node,qemu,lxc)."]
    pub cpu: Option<f64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used disk space in bytes (when type in storage), used root image spave for VMs (type in qemu,lxc)."]
    pub disk: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HA service status (for HA managed VMs)."]
    pub hastate: Option<String>,
    #[doc = "Resource id."]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Support level (when type == node)."]
    pub level: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of available CPUs (when type in node,qemu,lxc)."]
    pub maxcpu: Option<f64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Storage size in bytes (when type in storage), root image size for VMs (type in qemu,lxc)."]
    pub maxdisk: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of available memory in bytes (when type in node,qemu,lxc)."]
    pub maxmem: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used memory in bytes (when type in node,qemu,lxc)."]
    pub mem: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Name of the resource."]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The cluster node name (when type in node,storage,qemu,lxc)."]
    pub node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "More specific type, if available."]
    pub plugintype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The pool name (when type in pool,qemu,lxc)."]
    pub pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Resource type dependent status."]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The storage identifier (when type == storage)."]
    pub storage: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Resource type."]
    pub ty: Type,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node uptime in seconds (when type in node,qemu,lxc)."]
    pub uptime: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The numerical vmid (when type in qemu,lxc)."]
    pub vmid: Option<crate::types::VmId>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ResourcesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Resources index (cluster wide)."]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
