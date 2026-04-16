#[derive(Debug, Clone)]
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
impl<T> ResourcesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Resources index (cluster wide)."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(id: String, ty: Type2) -> Self {
        Self {
            id,
            ty,
            cgroup_mode: ::std::default::Default::default(),
            content: ::std::default::Default::default(),
            cpu: ::std::default::Default::default(),
            disk: ::std::default::Default::default(),
            diskread: ::std::default::Default::default(),
            diskwrite: ::std::default::Default::default(),
            hastate: ::std::default::Default::default(),
            level: ::std::default::Default::default(),
            lock: ::std::default::Default::default(),
            maxcpu: ::std::default::Default::default(),
            maxdisk: ::std::default::Default::default(),
            maxmem: ::std::default::Default::default(),
            mem: ::std::default::Default::default(),
            memhost: ::std::default::Default::default(),
            name: ::std::default::Default::default(),
            netin: ::std::default::Default::default(),
            netout: ::std::default::Default::default(),
            network: ::std::default::Default::default(),
            network_type: ::std::default::Default::default(),
            node: ::std::default::Default::default(),
            plugintype: ::std::default::Default::default(),
            pool: ::std::default::Default::default(),
            protocol: ::std::default::Default::default(),
            sdn: ::std::default::Default::default(),
            status: ::std::default::Default::default(),
            storage: ::std::default::Default::default(),
            tags: ::std::default::Default::default(),
            template: ::std::default::Default::default(),
            uptime: ::std::default::Default::default(),
            vmid: ::std::default::Default::default(),
            zone_type: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
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
    #[doc = "The cgroup mode the node operates under (for type 'node')."]
    #[doc = ""]
    pub cgroup_mode: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allowed storage content types (for type 'storage')."]
    #[doc = ""]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CPU utilization (for types 'node', 'qemu' and 'lxc')."]
    #[doc = ""]
    pub cpu: Option<CpuNum>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used disk space in bytes (for type 'storage'), used root image space for VMs (for types 'qemu' and 'lxc')."]
    #[doc = ""]
    pub disk: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The number of bytes the guest read from its block devices since the guest was started. This info is not available for all storage types. (for types 'qemu' and 'lxc')"]
    #[doc = ""]
    pub diskread: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The number of bytes the guest wrote to its block devices since the guest was started. This info is not available for all storage types. (for types 'qemu' and 'lxc')"]
    #[doc = ""]
    pub diskwrite: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HA service status (for HA managed VMs)."]
    #[doc = ""]
    pub hastate: Option<String>,
    #[doc = "Resource id."]
    #[doc = ""]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Support level (for type 'node')."]
    #[doc = ""]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The guest's current config lock (for types 'qemu' and 'lxc')"]
    #[doc = ""]
    pub lock: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of available CPUs (for types 'node', 'qemu' and 'lxc')."]
    #[doc = ""]
    pub maxcpu: Option<MaxcpuNum>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Storage size in bytes (for type 'storage'), root image size for VMs (for types 'qemu' and 'lxc')."]
    #[doc = ""]
    pub maxdisk: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of available memory in bytes (for types 'node', 'qemu' and 'lxc')."]
    #[doc = ""]
    pub maxmem: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used memory in bytes (for types 'node', 'qemu' and 'lxc')."]
    #[doc = ""]
    pub mem: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used memory in bytes from the point of view of the host (for types 'qemu')."]
    #[doc = ""]
    pub memhost: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Name of the resource."]
    #[doc = ""]
    pub name: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The amount of traffic in bytes that was sent to the guest over the network since it was started. (for types 'qemu' and 'lxc')"]
    #[doc = ""]
    pub netin: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The amount of traffic in bytes that was sent from the guest over the network since it was started. (for types 'qemu' and 'lxc')"]
    #[doc = ""]
    pub netout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The name of a Network entity (for type 'network')."]
    #[doc = ""]
    pub network: Option<String>,
    #[serde(rename = "network-type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The type of network resource (for type 'network')."]
    #[doc = ""]
    pub network_type: Option<NetworkType>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The cluster node name (for types 'node', 'storage', 'qemu', and 'lxc')."]
    #[doc = ""]
    pub node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "More specific type, if available."]
    #[doc = ""]
    pub plugintype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The pool name (for types 'pool', 'qemu' and 'lxc')."]
    #[doc = ""]
    pub pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The protocol of a fabric (for type 'network', network-type 'fabric')."]
    #[doc = ""]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The name of an SDN entity (for type 'sdn')"]
    #[doc = ""]
    pub sdn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Resource type dependent status."]
    #[doc = ""]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The storage identifier (for type 'storage')."]
    #[doc = ""]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The guest's tags (for types 'qemu' and 'lxc')"]
    #[doc = ""]
    pub tags: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determines if the guest is a template. (for types 'qemu' and 'lxc')"]
    #[doc = ""]
    pub template: Option<bool>,
    #[serde(rename = "type")]
    #[doc = "Resource type."]
    #[doc = ""]
    pub ty: Type2,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Uptime of node or virtual guest in seconds (for types 'node', 'qemu' and 'lxc')."]
    #[doc = ""]
    pub uptime: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The numerical vmid (for types 'qemu' and 'lxc')."]
    #[doc = ""]
    pub vmid: Option<VmidInt>,
    #[serde(rename = "zone-type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The type of an SDN zone (for type 'sdn')."]
    #[doc = ""]
    pub zone_type: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Resource type."]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The type of network resource (for type 'network')."]
#[doc = ""]
pub enum NetworkType {
    #[serde(rename = "fabric")]
    Fabric,
    #[serde(rename = "zone")]
    Zone,
}
impl TryFrom<&str> for NetworkType {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "fabric" => Ok(Self::Fabric),
            "zone" => Ok(Self::Zone),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Resource type."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "sdn")]
    Sdn,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "vm")]
    Vm,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "node" => Ok(Self::Node),
            "sdn" => Ok(Self::Sdn),
            "storage" => Ok(Self::Storage),
            "vm" => Ok(Self::Vm),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Resource type."]
#[doc = ""]
pub enum Type2 {
    #[serde(rename = "lxc")]
    Lxc,
    #[serde(rename = "network")]
    Network,
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
}
impl TryFrom<&str> for Type2 {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "lxc" => Ok(Self::Lxc),
            "network" => Ok(Self::Network),
            "node" => Ok(Self::Node),
            "openvz" => Ok(Self::Openvz),
            "pool" => Ok(Self::Pool),
            "qemu" => Ok(Self::Qemu),
            "sdn" => Ok(Self::Sdn),
            "storage" => Ok(Self::Storage),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VmidInt(i128);
impl crate::types::bounded_integer::BoundedInteger for VmidInt {
    const MIN: Option<i128> = Some(100i128);
    const MAX: Option<i128> = Some(999999999i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 100 and 999999999";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for VmidInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for VmidInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VmidInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CpuNum(f64);
impl crate::types::bounded_number::BoundedNumber for CpuNum {
    const MIN: Option<f64> = Some(0f64);
    const MAX: Option<f64> = None::<f64>;
    const DEFAULT: Option<f64> = None::<f64>;
    const TYPE_DESCRIPTION: &'static str = "an number greater than or equal to 0";
    fn get(&self) -> f64 {
        self.0
    }
    fn new(value: f64) -> Result<Self, crate::types::bounded_number::BoundedNumberError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<f64> for CpuNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value)
    }
}
impl std::convert::TryFrom<f32> for CpuNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i32> for CpuNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i64> for CpuNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl ::serde::Serialize for CpuNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_number::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CpuNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_number::deserialize_bounded_number(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MaxcpuNum(f64);
impl crate::types::bounded_number::BoundedNumber for MaxcpuNum {
    const MIN: Option<f64> = Some(0f64);
    const MAX: Option<f64> = None::<f64>;
    const DEFAULT: Option<f64> = None::<f64>;
    const TYPE_DESCRIPTION: &'static str = "an number greater than or equal to 0";
    fn get(&self) -> f64 {
        self.0
    }
    fn new(value: f64) -> Result<Self, crate::types::bounded_number::BoundedNumberError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<f64> for MaxcpuNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value)
    }
}
impl std::convert::TryFrom<f32> for MaxcpuNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i32> for MaxcpuNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i64> for MaxcpuNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl ::serde::Serialize for MaxcpuNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_number::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MaxcpuNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_number::deserialize_bounded_number(deserializer)
    }
}
