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
    pub fn new(port: u64, server: String, ty: Type) -> Self {
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
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "InfluxDB max-body-size in bytes. Requests are batched up to this size."]
    #[doc = ""]
    pub max_body_size: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU for metrics transmission over UDP"]
    #[doc = ""]
    pub mtu: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The InfluxDB organization. Only necessary when using the http v2 api. Has no meaning when using v2 compatibility api."]
    #[doc = ""]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "root graphite path (ex: proxmox.mycluster.mykey)"]
    #[doc = ""]
    pub path: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "server network port"]
    #[doc = ""]
    pub port: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Protocol to send graphite data. TCP or UDP (default)"]
    #[doc = ""]
    pub proto: Option<Proto>,
    #[doc = "server dns name or IP address"]
    #[doc = ""]
    pub server: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "graphite TCP socket timeout (default=1)"]
    #[doc = ""]
    pub timeout: Option<u64>,
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
    pub fn new(port: u64, server: String) -> Self {
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
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
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
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "InfluxDB max-body-size in bytes. Requests are batched up to this size."]
    #[doc = ""]
    pub max_body_size: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU for metrics transmission over UDP"]
    #[doc = ""]
    pub mtu: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The InfluxDB organization. Only necessary when using the http v2 api. Has no meaning when using v2 compatibility api."]
    #[doc = ""]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "root graphite path (ex: proxmox.mycluster.mykey)"]
    #[doc = ""]
    pub path: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "server network port"]
    #[doc = ""]
    pub port: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Protocol to send graphite data. TCP or UDP (default)"]
    #[doc = ""]
    pub proto: Option<Proto>,
    #[doc = "server dns name or IP address"]
    #[doc = ""]
    pub server: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "graphite TCP socket timeout (default=1)"]
    #[doc = ""]
    pub timeout: Option<u64>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
pub enum Influxdbproto {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "udp")]
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
impl Default for Influxdbproto {
    fn default() -> Self {
        Self::Udp
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
