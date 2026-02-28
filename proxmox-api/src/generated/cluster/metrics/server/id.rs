#[derive(Debug, Clone)]
pub struct IdClient<T> {
    client: T,
    path: String,
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Remove Metric server."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read metric server configuration."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new external metric server config"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update metric server configuration."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(port: PortInt, server: String, ty: Type) -> Self {
        Self {
            port,
            server,
            ty,
            api_path_prefix: Default::default(),
            bucket: Default::default(),
            disable: Default::default(),
            influxdbproto: Default::default(),
            max_body_size: Default::default(),
            mtu: Default::default(),
            organization: Default::default(),
            path: Default::default(),
            proto: Default::default(),
            timeout: Default::default(),
            token: Default::default(),
            verify_certificate: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(rename = "api-path-prefix")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "An API path prefix inserted between '\\\\<host\\\\>:\\\\<port\\\\>/' and '/api2/'. Can be useful if the InfluxDB service runs behind a reverse proxy."]
    #[doc = ""]
    pub api_path_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The InfluxDB bucket/db. Only necessary when using the http v2 api."]
    #[doc = ""]
    pub bucket: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Flag to disable the plugin."]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub influxdbproto: Option<Influxdbproto>,
    #[serde(rename = "max-body-size")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "InfluxDB max-body-size in bytes. Requests are batched up to this size."]
    #[doc = ""]
    pub max_body_size: Option<MaxBodySizeInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU for metrics transmission over UDP"]
    #[doc = ""]
    pub mtu: Option<MtuInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The InfluxDB organization. Only necessary when using the http v2 api. Has no meaning when using v2 compatibility api."]
    #[doc = ""]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "root graphite path (ex: proxmox.mycluster.mykey)"]
    #[doc = ""]
    pub path: Option<String>,
    #[doc = "server network port"]
    #[doc = ""]
    pub port: PortInt,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Protocol to send graphite data. TCP or UDP (default)"]
    #[doc = ""]
    pub proto: Option<Proto>,
    #[doc = "server dns name or IP address"]
    #[doc = ""]
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "graphite TCP socket timeout (default=1)"]
    #[doc = ""]
    pub timeout: Option<TimeoutInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The InfluxDB access token. Only necessary when using the http v2 api. If the v2 compatibility api is used, use 'user:password' instead."]
    #[doc = ""]
    pub token: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Plugin type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(rename = "verify-certificate")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set to 0 to disable certificate verification for https endpoints."]
    #[doc = ""]
    pub verify_certificate: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PutParams {
    pub fn new(port: PortInt, server: String) -> Self {
        Self {
            port,
            server,
            api_path_prefix: Default::default(),
            bucket: Default::default(),
            delete: Default::default(),
            digest: Default::default(),
            disable: Default::default(),
            influxdbproto: Default::default(),
            max_body_size: Default::default(),
            mtu: Default::default(),
            organization: Default::default(),
            path: Default::default(),
            proto: Default::default(),
            timeout: Default::default(),
            token: Default::default(),
            verify_certificate: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(rename = "api-path-prefix")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "An API path prefix inserted between '\\\\<host\\\\>:\\\\<port\\\\>/' and '/api2/'. Can be useful if the InfluxDB service runs behind a reverse proxy."]
    #[doc = ""]
    pub api_path_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The InfluxDB bucket/db. Only necessary when using the http v2 api."]
    #[doc = ""]
    pub bucket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<DeleteStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Flag to disable the plugin."]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub influxdbproto: Option<Influxdbproto>,
    #[serde(rename = "max-body-size")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "InfluxDB max-body-size in bytes. Requests are batched up to this size."]
    #[doc = ""]
    pub max_body_size: Option<MaxBodySizeInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU for metrics transmission over UDP"]
    #[doc = ""]
    pub mtu: Option<MtuInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The InfluxDB organization. Only necessary when using the http v2 api. Has no meaning when using v2 compatibility api."]
    #[doc = ""]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "root graphite path (ex: proxmox.mycluster.mykey)"]
    #[doc = ""]
    pub path: Option<String>,
    #[doc = "server network port"]
    #[doc = ""]
    pub port: PortInt,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Protocol to send graphite data. TCP or UDP (default)"]
    #[doc = ""]
    pub proto: Option<Proto>,
    #[doc = "server dns name or IP address"]
    #[doc = ""]
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "graphite TCP socket timeout (default=1)"]
    #[doc = ""]
    pub timeout: Option<TimeoutInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The InfluxDB access token. Only necessary when using the http v2 api. If the v2 compatibility api is used, use 'user:password' instead."]
    #[doc = ""]
    pub token: Option<String>,
    #[serde(rename = "verify-certificate")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set to 0 to disable certificate verification for https endpoints."]
    #[doc = ""]
    pub verify_certificate: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
pub enum Influxdbproto {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "udp")]
    #[default]
    Udp,
}
impl TryFrom<&str> for Influxdbproto {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "http" => Ok(Self::Http),
            "https" => Ok(Self::Https),
            "udp" => Ok(Self::Udp),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Protocol to send graphite data. TCP or UDP (default)"]
#[doc = ""]
pub enum Proto {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}
impl TryFrom<&str> for Proto {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "tcp" => Ok(Self::Tcp),
            "udp" => Ok(Self::Udp),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Plugin type."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "graphite")]
    Graphite,
    #[serde(rename = "influxdb")]
    Influxdb,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "graphite" => Ok(Self::Graphite),
            "influxdb" => Ok(Self::Influxdb),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MaxBodySizeInt(i128);
impl crate::types::bounded_integer::BoundedInteger for MaxBodySizeInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(25000000i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 1";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for MaxBodySizeInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for MaxBodySizeInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MaxBodySizeInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MtuInt(i128);
impl crate::types::bounded_integer::BoundedInteger for MtuInt {
    const MIN: Option<i128> = Some(512i128);
    const MAX: Option<i128> = Some(65536i128);
    const DEFAULT: Option<i128> = Some(1500i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 512 and 65536";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for MtuInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for MtuInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MtuInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PortInt(i128);
impl crate::types::bounded_integer::BoundedInteger for PortInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(65536i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 65536";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for PortInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for PortInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PortInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeoutInt(i128);
impl crate::types::bounded_integer::BoundedInteger for TimeoutInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(1i128);
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
pub struct DeleteStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DeleteStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(4096usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 4096";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DeleteStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DeleteStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DeleteStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DigestStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DigestStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(64usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 64";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DigestStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DigestStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DigestStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
