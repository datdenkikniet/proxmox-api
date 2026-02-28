pub mod status;
#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Destroy pool"]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Pool index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Change POOL settings"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If true, destroys pool even if in use"]
    #[doc = ""]
    pub force: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Remove the erasure code profile. Defaults to true, if applicable."]
    #[doc = ""]
    pub remove_ecprofile: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Remove all pveceph-managed storages configured for this pool"]
    #[doc = ""]
    pub remove_storages: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The application of the pool."]
    #[doc = ""]
    pub application: Option<Application>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The rule to use for mapping object placement in the cluster."]
    #[doc = ""]
    pub crush_rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Minimum number of replicas per object"]
    #[doc = ""]
    pub min_size: Option<MinSizeInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The automatic PG scaling mode of the pool."]
    #[doc = ""]
    pub pg_autoscale_mode: Option<PgAutoscaleMode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of placement groups."]
    #[doc = ""]
    pub pg_num: Option<PgNumInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Minimal number of placement groups."]
    #[doc = ""]
    pub pg_num_min: Option<PgNumMinInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of replicas per object"]
    #[doc = ""]
    pub size: Option<SizeInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The estimated target size of the pool for the PG autoscaler."]
    #[doc = ""]
    pub target_size: Option<TargetSizeStr>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The estimated target ratio of the pool for the PG autoscaler."]
    #[doc = ""]
    pub target_size_ratio: Option<f64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The application of the pool."]
#[doc = ""]
pub enum Application {
    #[serde(rename = "cephfs")]
    Cephfs,
    #[serde(rename = "rbd")]
    Rbd,
    #[serde(rename = "rgw")]
    Rgw,
}
impl TryFrom<&str> for Application {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "cephfs" => Ok(Self::Cephfs),
            "rbd" => Ok(Self::Rbd),
            "rgw" => Ok(Self::Rgw),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The automatic PG scaling mode of the pool."]
#[doc = ""]
pub enum PgAutoscaleMode {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "warn")]
    Warn,
}
impl TryFrom<&str> for PgAutoscaleMode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            "warn" => Ok(Self::Warn),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MinSizeInt(i128);
impl crate::types::bounded_integer::BoundedInteger for MinSizeInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(7i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 7";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for MinSizeInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for MinSizeInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MinSizeInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PgNumInt(i128);
impl crate::types::bounded_integer::BoundedInteger for PgNumInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(32768i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 32768";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for PgNumInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for PgNumInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PgNumInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PgNumMinInt(i128);
impl crate::types::bounded_integer::BoundedInteger for PgNumMinInt {
    const MIN: Option<i128> = None::<i128>;
    const MAX: Option<i128> = Some(32768i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer less than or equal to 32768";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for PgNumMinInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for PgNumMinInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PgNumMinInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SizeInt(i128);
impl crate::types::bounded_integer::BoundedInteger for SizeInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(7i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 7";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for SizeInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for SizeInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SizeInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TargetSizeStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for TargetSizeStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("^(\\d+(\\.\\d+)?)([KMGT])?$");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"^(\\d+(\\.\\d+)?)([KMGT])?$\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for TargetSizeStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for TargetSizeStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for TargetSizeStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
