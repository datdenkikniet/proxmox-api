#[derive(Debug, Clone)]
pub struct RemoteMigrateClient<T> {
    client: T,
    path: String,
}
impl<T> RemoteMigrateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/remote_migrate"),
        }
    }
}
impl<T> RemoteMigrateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Migrate virtual machine to a remote cluster. Creates a new migration task. EXPERIMENTAL feature!"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(target_bridge: String, target_endpoint: String, target_storage: String) -> Self {
        Self {
            target_bridge,
            target_endpoint,
            target_storage,
            bwlimit: Default::default(),
            delete: Default::default(),
            online: Default::default(),
            target_vmid: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<BwlimitInt>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Delete the original VM and related data after successful migration. By default the original VM is kept on the source cluster in a stopped state."]
    #[doc = ""]
    pub delete: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use online/live migration if VM is running. Ignored if VM is stopped."]
    #[doc = ""]
    pub online: Option<bool>,
    #[serde(rename = "target-bridge")]
    #[doc = "Mapping from source to target bridges. Providing only a single bridge ID maps all source bridges to that bridge. Providing the special value '1' will map each source bridge to itself."]
    #[doc = ""]
    pub target_bridge: String,
    #[serde(rename = "target-endpoint")]
    #[doc = "Remote target endpoint"]
    #[doc = ""]
    pub target_endpoint: String,
    #[serde(rename = "target-storage")]
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    #[doc = ""]
    pub target_storage: String,
    #[serde(rename = "target-vmid")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The (unique) ID of the VM."]
    #[doc = ""]
    pub target_vmid: Option<TargetVmidInt>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BwlimitInt(i128);
impl crate::types::bounded_integer::BoundedInteger for BwlimitInt {
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
impl std::convert::TryFrom<i128> for BwlimitInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for BwlimitInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for BwlimitInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TargetVmidInt(i128);
impl crate::types::bounded_integer::BoundedInteger for TargetVmidInt {
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
impl std::convert::TryFrom<i128> for TargetVmidInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for TargetVmidInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for TargetVmidInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
