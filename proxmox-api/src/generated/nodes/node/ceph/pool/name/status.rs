#[derive(Debug, Clone)]
pub struct StatusClient<T> {
    client: T,
    path: String,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/status"),
        }
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Show the current pool status."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct AutoscaleStatusGetOutputAutoscaleStatus {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(
        fast_read: bool,
        hashpspool: bool,
        id: i64,
        name: String,
        nodeep_scrub: bool,
        nodelete: bool,
        nopgchange: bool,
        noscrub: bool,
        nosizechange: bool,
        pgp_num: i64,
        use_gmt_hitset: bool,
        write_fadvise_dontneed: bool,
    ) -> Self {
        Self {
            fast_read,
            hashpspool,
            id,
            name,
            nodeep_scrub,
            nodelete,
            nopgchange,
            noscrub,
            nosizechange,
            pgp_num,
            use_gmt_hitset,
            write_fadvise_dontneed,
            application: Default::default(),
            autoscale_status: Default::default(),
            crush_rule: Default::default(),
            min_size: Default::default(),
            pg_autoscale_mode: Default::default(),
            pg_num: Default::default(),
            pg_num_min: Default::default(),
            size: Default::default(),
            statistics: Default::default(),
            target_size: Default::default(),
            target_size_ratio: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The application of the pool."]
    #[doc = ""]
    pub application: Option<Application>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub autoscale_status: Option<AutoscaleStatusGetOutputAutoscaleStatus>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The rule to use for mapping object placement in the cluster."]
    #[doc = ""]
    pub crush_rule: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub fast_read: bool,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub hashpspool: bool,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Minimum number of replicas per object"]
    #[doc = ""]
    pub min_size: Option<MinSizeInt>,
    #[doc = "The name of the pool. It must be unique."]
    #[doc = ""]
    pub name: String,
    #[serde(rename = "nodeep-scrub")]
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub nodeep_scrub: bool,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub nodelete: bool,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub nopgchange: bool,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub noscrub: bool,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub nosizechange: bool,
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
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub pgp_num: i64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of replicas per object"]
    #[doc = ""]
    pub size: Option<SizeInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub statistics: Option<StatisticsGetOutputStatistics>,
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
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub use_gmt_hitset: bool,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub write_fadvise_dontneed: bool,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If enabled, will display additional data(eg. statistics)."]
    #[doc = ""]
    pub verbose: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct StatisticsGetOutputStatistics {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "The application of the pool."]
#[doc = ""]
pub enum Application {
    #[serde(rename = "cephfs")]
    Cephfs,
    #[serde(rename = "rbd")]
    #[default]
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "The automatic PG scaling mode of the pool."]
#[doc = ""]
pub enum PgAutoscaleMode {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "warn")]
    #[default]
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
    const DEFAULT: Option<i128> = Some(2i128);
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
    const DEFAULT: Option<i128> = Some(128i128);
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
    const DEFAULT: Option<i128> = Some(3i128);
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
