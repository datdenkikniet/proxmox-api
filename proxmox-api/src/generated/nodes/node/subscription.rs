#[derive(Debug, Clone)]
pub struct SubscriptionClient<T> {
    client: T,
    path: String,
}
impl<T> SubscriptionClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/subscription"),
        }
    }
}
impl<T> SubscriptionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete subscription key of this node."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> SubscriptionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read subscription info."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> SubscriptionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update subscription info."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> SubscriptionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set subscription key."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl GetOutput {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            checktime: ::std::default::Default::default(),
            key: ::std::default::Default::default(),
            level: ::std::default::Default::default(),
            message: ::std::default::Default::default(),
            nextduedate: ::std::default::Default::default(),
            productname: ::std::default::Default::default(),
            regdate: ::std::default::Default::default(),
            serverid: ::std::default::Default::default(),
            signature: ::std::default::Default::default(),
            sockets: ::std::default::Default::default(),
            url: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Timestamp of the last check done."]
    #[doc = ""]
    pub checktime: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The subscription key, if set and permitted to access."]
    #[doc = ""]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A short code for the subscription level."]
    #[doc = ""]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A more human readable status message."]
    #[doc = ""]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Next due date of the set subscription."]
    #[doc = ""]
    pub nextduedate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Human readable productname of the set subscription."]
    #[doc = ""]
    pub productname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Register date of the set subscription."]
    #[doc = ""]
    pub regdate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The server ID, if permitted to access."]
    #[doc = ""]
    pub serverid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Signature for offline keys"]
    #[doc = ""]
    pub signature: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The number of sockets for this host."]
    #[doc = ""]
    pub sockets: Option<i64>,
    #[doc = "The current subscription status."]
    #[doc = ""]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL to the web shop."]
    #[doc = ""]
    pub url: Option<String>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Always connect to server, even if local cache is still valid."]
    #[doc = ""]
    pub force: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PutParams {
    pub fn new(key: KeyStr) -> Self {
        Self {
            key,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[doc = "Proxmox VE subscription key"]
    #[doc = ""]
    pub key: KeyStr,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The current subscription status."]
#[doc = ""]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "new")]
    New,
    #[serde(rename = "notfound")]
    Notfound,
    #[serde(rename = "suspended")]
    Suspended,
}
impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "active" => Ok(Self::Active),
            "expired" => Ok(Self::Expired),
            "invalid" => Ok(Self::Invalid),
            "new" => Ok(Self::New),
            "notfound" => Ok(Self::Notfound),
            "suspended" => Ok(Self::Suspended),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct KeyStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for KeyStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(32usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("\\s*pve([1248])([cbsp])-[0-9a-f]{10}\\s*");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"\\s*pve([1248])([cbsp])-[0-9a-f]{10}\\s*\" and length at most 32";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for KeyStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for KeyStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for KeyStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
