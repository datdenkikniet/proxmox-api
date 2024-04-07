pub struct MetadataClient<T> {
    client: T,
    path: String,
}
impl<T> MetadataClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/metadata"),
        }
    }
}
impl<T> MetadataClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get ceph metadata."]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutput {
    pub fn new(
        mds: MdsGetOutputMds,
        mgr: MgrGetOutputMgr,
        mon: MonGetOutputMon,
        node: NodeGetOutputNode,
    ) -> Self {
        Self {
            mds,
            mgr,
            mon,
            node,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[doc = "Metadata servers configured in the cluster and their properties."]
    pub mds: MdsGetOutputMds,
    #[doc = "Managers configured in the cluster and their properties."]
    pub mgr: MgrGetOutputMgr,
    #[doc = "Monitors configured in the cluster and their properties."]
    pub mon: MonGetOutputMon,
    #[doc = "Ceph version installed on the nodes."]
    pub node: NodeGetOutputNode,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scope: Option<Scope>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl IdGetOutputMdsId {
    pub fn new(
        addr: String,
        ceph_release: String,
        ceph_version: String,
        ceph_version_short: String,
        hostname: String,
        mem_swap_kb: u64,
        mem_total_kb: u64,
        name: String,
    ) -> Self {
        Self {
            addr,
            ceph_release,
            ceph_version,
            ceph_version_short,
            hostname,
            mem_swap_kb,
            mem_total_kb,
            name,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct IdGetOutputMdsId {
    #[doc = "Bind addresses and ports."]
    pub addr: String,
    #[doc = "Ceph release codename currently used."]
    pub ceph_release: String,
    #[doc = "Version info currently used by the service."]
    pub ceph_version: String,
    #[doc = "Short version (numerical) info currently used by the service."]
    pub ceph_version_short: String,
    #[doc = "Hostname on which the service is running."]
    pub hostname: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory of the service currently in swap."]
    pub mem_swap_kb: u64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory consumption of the service."]
    pub mem_total_kb: u64,
    #[doc = "Name of the service instance."]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl IdGetOutputMgrId {
    pub fn new(
        addr: String,
        ceph_release: String,
        ceph_version: String,
        ceph_version_short: String,
        hostname: String,
        mem_swap_kb: u64,
        mem_total_kb: u64,
        name: String,
    ) -> Self {
        Self {
            addr,
            ceph_release,
            ceph_version,
            ceph_version_short,
            hostname,
            mem_swap_kb,
            mem_total_kb,
            name,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct IdGetOutputMgrId {
    #[doc = "Bind address"]
    pub addr: String,
    #[doc = "Ceph release codename currently used."]
    pub ceph_release: String,
    #[doc = "Version info currently used by the service."]
    pub ceph_version: String,
    #[doc = "Short version (numerical) info currently used by the service."]
    pub ceph_version_short: String,
    #[doc = "Hostname on which the service is running."]
    pub hostname: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory of the service currently in swap."]
    pub mem_swap_kb: u64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory consumption of the service."]
    pub mem_total_kb: u64,
    #[doc = "Name of the service instance."]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl IdGetOutputMonId {
    pub fn new(
        addrs: String,
        ceph_release: String,
        ceph_version: String,
        ceph_version_short: String,
        hostname: String,
        mem_swap_kb: u64,
        mem_total_kb: u64,
        name: String,
    ) -> Self {
        Self {
            addrs,
            ceph_release,
            ceph_version,
            ceph_version_short,
            hostname,
            mem_swap_kb,
            mem_total_kb,
            name,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct IdGetOutputMonId {
    #[doc = "Bind addresses and ports."]
    pub addrs: String,
    #[doc = "Ceph release codename currently used."]
    pub ceph_release: String,
    #[doc = "Version info currently used by the service."]
    pub ceph_version: String,
    #[doc = "Short version (numerical) info currently used by the service."]
    pub ceph_version_short: String,
    #[doc = "Hostname on which the service is running."]
    pub hostname: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory of the service currently in swap."]
    pub mem_swap_kb: u64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory consumption of the service."]
    pub mem_total_kb: u64,
    #[doc = "Name of the service instance."]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl MdsGetOutputMds {
    pub fn new(_id: IdGetOutputMdsId) -> Self {
        Self {
            _id,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct MdsGetOutputMds {
    #[serde(rename = "{id}")]
    #[doc = "Useful properties are listed, but not the full list."]
    pub _id: IdGetOutputMdsId,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl MgrGetOutputMgr {
    pub fn new(_id: IdGetOutputMgrId) -> Self {
        Self {
            _id,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct MgrGetOutputMgr {
    #[serde(rename = "{id}")]
    #[doc = "Useful properties are listed, but not the full list."]
    pub _id: IdGetOutputMgrId,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl MonGetOutputMon {
    pub fn new(_id: IdGetOutputMonId) -> Self {
        Self {
            _id,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct MonGetOutputMon {
    #[serde(rename = "{id}")]
    #[doc = "Useful properties are listed, but not the full list."]
    pub _id: IdGetOutputMonId,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl NodeGetOutputNode {
    pub fn new(_node: NodeGetOutputNodeNode) -> Self {
        Self {
            _node,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct NodeGetOutputNode {
    #[serde(rename = "{node}")]
    pub _node: NodeGetOutputNodeNode,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl NodeGetOutputNodeNode {
    pub fn new(buildcommit: String, version: VersionGetOutputNodeNodeVersion) -> Self {
        Self {
            buildcommit,
            version,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct NodeGetOutputNodeNode {
    #[doc = "GIT commit used for the build."]
    pub buildcommit: String,
    #[doc = "Version info."]
    pub version: VersionGetOutputNodeNodeVersion,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl VersionGetOutputNodeNodeVersion {
    pub fn new(str: String) -> Self {
        Self {
            str,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct VersionGetOutputNodeNodeVersion {
    #[doc = "Version as single string."]
    pub str: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Scope {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "versions")]
    Versions,
}
impl Default for Scope {
    fn default() -> Self {
        Self::All
    }
}
