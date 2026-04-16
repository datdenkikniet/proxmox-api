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
            api_path_prefix: ::std::default::Default::default(),
            bucket: ::std::default::Default::default(),
            disable: ::std::default::Default::default(),
            influxdbproto: ::std::default::Default::default(),
            max_body_size: ::std::default::Default::default(),
            mtu: ::std::default::Default::default(),
            organization: ::std::default::Default::default(),
            otel_compression: ::std::default::Default::default(),
            otel_headers: ::std::default::Default::default(),
            otel_max_body_size: ::std::default::Default::default(),
            otel_path: ::std::default::Default::default(),
            otel_protocol: ::std::default::Default::default(),
            otel_resource_attributes: ::std::default::Default::default(),
            otel_timeout: ::std::default::Default::default(),
            otel_verify_ssl: ::std::default::Default::default(),
            path: ::std::default::Default::default(),
            proto: ::std::default::Default::default(),
            timeout: ::std::default::Default::default(),
            token: ::std::default::Default::default(),
            verify_certificate: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
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
    #[serde(rename = "otel-compression")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Compression algorithm for requests"]
    #[doc = ""]
    pub otel_compression: Option<OtelCompression>,
    #[serde(rename = "otel-headers")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Custom HTTP headers (JSON format, base64 encoded)"]
    #[doc = ""]
    pub otel_headers: Option<OtelHeadersStr>,
    #[serde(rename = "otel-max-body-size")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum request body size in bytes"]
    #[doc = ""]
    pub otel_max_body_size: Option<OtelMaxBodySizeInt>,
    #[serde(rename = "otel-path")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OTLP endpoint path"]
    #[doc = ""]
    pub otel_path: Option<String>,
    #[serde(rename = "otel-protocol")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HTTP protocol"]
    #[doc = ""]
    pub otel_protocol: Option<OtelProtocol>,
    #[serde(rename = "otel-resource-attributes")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Additional resource attributes as JSON, base64 encoded"]
    #[doc = ""]
    pub otel_resource_attributes: Option<OtelResourceAttributesStr>,
    #[serde(rename = "otel-timeout")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HTTP request timeout in seconds"]
    #[doc = ""]
    pub otel_timeout: Option<OtelTimeoutInt>,
    #[serde(rename = "otel-verify-ssl")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Verify SSL certificates"]
    #[doc = ""]
    pub otel_verify_ssl: Option<bool>,
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
            api_path_prefix: ::std::default::Default::default(),
            bucket: ::std::default::Default::default(),
            delete: ::std::default::Default::default(),
            digest: ::std::default::Default::default(),
            disable: ::std::default::Default::default(),
            influxdbproto: ::std::default::Default::default(),
            max_body_size: ::std::default::Default::default(),
            mtu: ::std::default::Default::default(),
            organization: ::std::default::Default::default(),
            otel_compression: ::std::default::Default::default(),
            otel_headers: ::std::default::Default::default(),
            otel_max_body_size: ::std::default::Default::default(),
            otel_path: ::std::default::Default::default(),
            otel_protocol: ::std::default::Default::default(),
            otel_resource_attributes: ::std::default::Default::default(),
            otel_timeout: ::std::default::Default::default(),
            otel_verify_ssl: ::std::default::Default::default(),
            path: ::std::default::Default::default(),
            proto: ::std::default::Default::default(),
            timeout: ::std::default::Default::default(),
            token: ::std::default::Default::default(),
            verify_certificate: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
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
    #[serde(rename = "otel-compression")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Compression algorithm for requests"]
    #[doc = ""]
    pub otel_compression: Option<OtelCompression>,
    #[serde(rename = "otel-headers")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Custom HTTP headers (JSON format, base64 encoded)"]
    #[doc = ""]
    pub otel_headers: Option<OtelHeadersStr>,
    #[serde(rename = "otel-max-body-size")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum request body size in bytes"]
    #[doc = ""]
    pub otel_max_body_size: Option<OtelMaxBodySizeInt>,
    #[serde(rename = "otel-path")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OTLP endpoint path"]
    #[doc = ""]
    pub otel_path: Option<String>,
    #[serde(rename = "otel-protocol")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HTTP protocol"]
    #[doc = ""]
    pub otel_protocol: Option<OtelProtocol>,
    #[serde(rename = "otel-resource-attributes")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Additional resource attributes as JSON, base64 encoded"]
    #[doc = ""]
    pub otel_resource_attributes: Option<OtelResourceAttributesStr>,
    #[serde(rename = "otel-timeout")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HTTP request timeout in seconds"]
    #[doc = ""]
    pub otel_timeout: Option<OtelTimeoutInt>,
    #[serde(rename = "otel-verify-ssl")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Verify SSL certificates"]
    #[doc = ""]
    pub otel_verify_ssl: Option<bool>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Compression algorithm for requests"]
#[doc = ""]
pub enum OtelCompression {
    #[serde(rename = "gzip")]
    #[default]
    Gzip,
    #[serde(rename = "none")]
    None,
}
impl TryFrom<&str> for OtelCompression {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "gzip" => Ok(Self::Gzip),
            "none" => Ok(Self::None),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "HTTP protocol"]
#[doc = ""]
pub enum OtelProtocol {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    #[default]
    Https,
}
impl TryFrom<&str> for OtelProtocol {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "http" => Ok(Self::Http),
            "https" => Ok(Self::Https),
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
    #[serde(rename = "opentelemetry")]
    Opentelemetry,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "graphite" => Ok(Self::Graphite),
            "influxdb" => Ok(Self::Influxdb),
            "opentelemetry" => Ok(Self::Opentelemetry),
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
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MaxBodySizeInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
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
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MtuInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OtelMaxBodySizeInt(i128);
impl crate::types::bounded_integer::BoundedInteger for OtelMaxBodySizeInt {
    const MIN: Option<i128> = Some(1024i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(10000000i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 1024";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for OtelMaxBodySizeInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for OtelMaxBodySizeInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for OtelMaxBodySizeInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OtelTimeoutInt(i128);
impl crate::types::bounded_integer::BoundedInteger for OtelTimeoutInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(10i128);
    const DEFAULT: Option<i128> = Some(5i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 10";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for OtelTimeoutInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for OtelTimeoutInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for OtelTimeoutInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
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
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PortInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
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
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for TimeoutInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
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
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DeleteStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
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
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DigestStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OtelHeadersStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for OtelHeadersStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(1024usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 1024";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for OtelHeadersStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for OtelHeadersStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for OtelHeadersStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OtelResourceAttributesStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for OtelResourceAttributesStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(1024usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 1024";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for OtelResourceAttributesStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for OtelResourceAttributesStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for OtelResourceAttributesStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
