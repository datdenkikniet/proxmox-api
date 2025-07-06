#[derive(Debug, Clone)]
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
    #[doc = ""]
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
    #[doc = ""]
    pub mds: MdsGetOutputMds,
    #[doc = "Managers configured in the cluster and their properties."]
    #[doc = ""]
    pub mgr: MgrGetOutputMgr,
    #[doc = "Monitors configured in the cluster and their properties."]
    #[doc = ""]
    pub mon: MonGetOutputMon,
    #[doc = "Ceph version installed on the nodes."]
    #[doc = ""]
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
        mem_swap_kb: i64,
        mem_total_kb: i64,
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
    #[doc = ""]
    pub addr: String,
    #[doc = "Ceph release codename currently used."]
    #[doc = ""]
    pub ceph_release: String,
    #[doc = "Version info currently used by the service."]
    #[doc = ""]
    pub ceph_version: String,
    #[doc = "Short version (numerical) info currently used by the service."]
    #[doc = ""]
    pub ceph_version_short: String,
    #[doc = "Hostname on which the service is running."]
    #[doc = ""]
    pub hostname: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory of the service currently in swap."]
    #[doc = ""]
    pub mem_swap_kb: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory consumption of the service."]
    #[doc = ""]
    pub mem_total_kb: i64,
    #[doc = "Name of the service instance."]
    #[doc = ""]
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
        mem_swap_kb: i64,
        mem_total_kb: i64,
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
    #[doc = ""]
    pub addr: String,
    #[doc = "Ceph release codename currently used."]
    #[doc = ""]
    pub ceph_release: String,
    #[doc = "Version info currently used by the service."]
    #[doc = ""]
    pub ceph_version: String,
    #[doc = "Short version (numerical) info currently used by the service."]
    #[doc = ""]
    pub ceph_version_short: String,
    #[doc = "Hostname on which the service is running."]
    #[doc = ""]
    pub hostname: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory of the service currently in swap."]
    #[doc = ""]
    pub mem_swap_kb: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory consumption of the service."]
    #[doc = ""]
    pub mem_total_kb: i64,
    #[doc = "Name of the service instance."]
    #[doc = ""]
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
        mem_swap_kb: i64,
        mem_total_kb: i64,
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
    #[doc = ""]
    pub addrs: String,
    #[doc = "Ceph release codename currently used."]
    #[doc = ""]
    pub ceph_release: String,
    #[doc = "Version info currently used by the service."]
    #[doc = ""]
    pub ceph_version: String,
    #[doc = "Short version (numerical) info currently used by the service."]
    #[doc = ""]
    pub ceph_version_short: String,
    #[doc = "Hostname on which the service is running."]
    #[doc = ""]
    pub hostname: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory of the service currently in swap."]
    #[doc = ""]
    pub mem_swap_kb: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory consumption of the service."]
    #[doc = ""]
    pub mem_total_kb: i64,
    #[doc = "Name of the service instance."]
    #[doc = ""]
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
    #[doc = ""]
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
    #[doc = ""]
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
    #[doc = ""]
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
    #[doc = ""]
    pub buildcommit: String,
    #[doc = "Version info."]
    #[doc = ""]
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
    #[doc = ""]
    pub str: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
pub enum Scope {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "versions")]
    Versions,
}
impl TryFrom<&str> for Scope {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "all" => Ok(Self::All),
            "versions" => Ok(Self::Versions),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Scope {
    fn default() -> Self {
        Self::All
    }
}
