pub mod ips;
pub mod subnets;
#[derive(Debug, Clone)]
pub struct VnetClient<T> {
    client: T,
    path: String,
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, vnet: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, vnet),
        }
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete sdn vnet object configuration."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read sdn vnet configuration."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update sdn vnet object configuration."]
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display pending config."]
    #[doc = ""]
    pub pending: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display running config."]
    #[doc = ""]
    pub running: Option<bool>,
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
    #[doc = "alias name of the vnet"]
    #[doc = ""]
    pub alias: Option<AliasStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<DeleteStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "vlan or vxlan id"]
    #[doc = ""]
    pub tag: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow vm VLANs to pass through this vnet."]
    #[doc = ""]
    pub vlanaware: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "zone id"]
    #[doc = ""]
    pub zone: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AliasStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for AliasStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(256usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("(?^i:[\\(\\)-_.\\w\\d\\s]{0,256})");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"(?^i:[\\(\\)-_.\\w\\d\\s]{0,256})\" and length at most 256";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for AliasStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for AliasStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for AliasStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
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
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    pub fn subnets(&self) -> subnets::SubnetsClient<T> {
        subnets::SubnetsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    pub fn ips(&self) -> ips::IpsClient<T> {
        ips::IpsClient::<T>::new(self.client.clone(), &self.path)
    }
}
