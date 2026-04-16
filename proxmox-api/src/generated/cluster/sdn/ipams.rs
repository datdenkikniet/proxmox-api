pub mod ipam;
#[derive(Debug, Clone)]
pub struct IpamsClient<T> {
    client: T,
    path: String,
}
impl<T> IpamsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/ipams"),
        }
    }
}
impl<T> IpamsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN ipams index."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> IpamsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new sdn ipam object."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(ipam: String, ty: String) -> Self {
        Self {
            ipam,
            ty,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub ipam: String,
    #[serde(rename = "type")]
    pub ty: String,
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
    #[doc = "Only list sdn ipams of specific type"]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(ipam: String, ty: Type) -> Self {
        Self {
            ipam,
            ty,
            fingerprint: ::std::default::Default::default(),
            lock_token: ::std::default::Default::default(),
            section: ::std::default::Default::default(),
            token: ::std::default::Default::default(),
            url: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate SHA 256 fingerprint."]
    #[doc = ""]
    pub fingerprint: Option<FingerprintStr>,
    #[doc = "The SDN ipam object identifier."]
    #[doc = ""]
    pub ipam: String,
    #[serde(rename = "lock-token")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "the token for unlocking the global SDN configuration"]
    #[doc = ""]
    pub lock_token: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub section: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub token: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Plugin type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Only list sdn ipams of specific type"]
#[doc = ""]
pub enum Type {
    #[serde(rename = "netbox")]
    Netbox,
    #[serde(rename = "phpipam")]
    Phpipam,
    #[serde(rename = "pve")]
    Pve,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "netbox" => Ok(Self::Netbox),
            "phpipam" => Ok(Self::Phpipam),
            "pve" => Ok(Self::Pve),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FingerprintStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for FingerprintStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("([A-Fa-f0-9]{2}:){31}[A-Fa-f0-9]{2}");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"([A-Fa-f0-9]{2}:){31}[A-Fa-f0-9]{2}\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for FingerprintStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for FingerprintStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for FingerprintStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
impl<T> IpamsClient<T>
where
    T: crate::client::Client,
{
    pub fn ipam(&self, ipam: &str) -> ipam::IpamClient<T> {
        ipam::IpamClient::<T>::new(self.client.clone(), &self.path, ipam)
    }
}
