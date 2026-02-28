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
    #[doc = ""]
    pub cgroup_mode: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allowed storage content types (when type == storage)."]
    #[doc = ""]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CPU utilization (when type in node,qemu,lxc)."]
    #[doc = ""]
    pub cpu: Option<CpuNum>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used disk space in bytes (when type in storage), used root image spave for VMs (type in qemu,lxc)."]
    #[doc = ""]
    pub disk: Option<DiskInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HA service status (for HA managed VMs)."]
    #[doc = ""]
    pub hastate: Option<String>,
    #[doc = "Resource id."]
    #[doc = ""]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Support level (when type == node)."]
    #[doc = ""]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of available CPUs (when type in node,qemu,lxc)."]
    #[doc = ""]
    pub maxcpu: Option<MaxcpuNum>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Storage size in bytes (when type in storage), root image size for VMs (type in qemu,lxc)."]
    #[doc = ""]
    pub maxdisk: Option<MaxdiskInt>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of available memory in bytes (when type in node,qemu,lxc)."]
    #[doc = ""]
    pub maxmem: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used memory in bytes (when type in node,qemu,lxc)."]
    #[doc = ""]
    pub mem: Option<MemInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Name of the resource."]
    #[doc = ""]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The cluster node name (when type in node,storage,qemu,lxc)."]
    #[doc = ""]
    pub node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "More specific type, if available."]
    #[doc = ""]
    pub plugintype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The pool name (when type in pool,qemu,lxc)."]
    #[doc = ""]
    pub pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Resource type dependent status."]
    #[doc = ""]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The storage identifier (when type == storage)."]
    #[doc = ""]
    pub storage: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Resource type."]
    #[doc = ""]
    pub ty: Type2,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node uptime in seconds (when type in node,qemu,lxc)."]
    #[doc = ""]
    pub uptime: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The numerical vmid (when type in qemu,lxc)."]
    #[doc = ""]
    pub vmid: Option<VmidInt>,
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
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
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
pub struct DiskInt(i128);
impl crate::types::bounded_integer::BoundedInteger for DiskInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for DiskInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for DiskInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DiskInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MaxdiskInt(i128);
impl crate::types::bounded_integer::BoundedInteger for MaxdiskInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for MaxdiskInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for MaxdiskInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MaxdiskInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MemInt(i128);
impl crate::types::bounded_integer::BoundedInteger for MemInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for MemInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for MemInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MemInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
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
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VmidInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
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
        crate::types::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CpuNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_number(deserializer)
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
        crate::types::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MaxcpuNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_number(deserializer)
    }
}
