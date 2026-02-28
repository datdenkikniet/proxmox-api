#[derive(Debug, Clone)]
pub struct StartClient<T> {
    client: T,
    path: String,
}
impl<T> StartClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/start"),
        }
    }
}
impl<T> StartClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Start virtual machine."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(rename = "force-cpu")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override QEMU's -cpu argument with the given string."]
    #[doc = ""]
    pub force_cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the QEMU machine."]
    #[doc = ""]
    pub machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The cluster node name."]
    #[doc = ""]
    pub migratedfrom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CIDR of the (sub) network that is used for migration."]
    #[doc = ""]
    pub migration_network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Migration traffic is encrypted using an SSH tunnel by default. On secure, completely private networks this can be disabled to increase performance."]
    #[doc = ""]
    pub migration_type: Option<MigrationType>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[doc = ""]
    pub skiplock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Some command save/restore state from this location."]
    #[doc = ""]
    pub stateuri: Option<StateuriStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    #[doc = ""]
    pub targetstorage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Wait maximal timeout seconds."]
    #[doc = ""]
    pub timeout: Option<TimeoutInt>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Migration traffic is encrypted using an SSH tunnel by default. On secure, completely private networks this can be disabled to increase performance."]
#[doc = ""]
pub enum MigrationType {
    #[serde(rename = "insecure")]
    Insecure,
    #[serde(rename = "secure")]
    Secure,
}
impl TryFrom<&str> for MigrationType {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "insecure" => Ok(Self::Insecure),
            "secure" => Ok(Self::Secure),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeoutInt(i128);
impl crate::types::bounded_integer::BoundedInteger for TimeoutInt {
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
impl std::convert::TryFrom<i128> for TimeoutInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for TimeoutInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for TimeoutInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StateuriStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for StateuriStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(128usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 128";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for StateuriStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for StateuriStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for StateuriStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
