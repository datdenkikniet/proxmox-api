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
            checktime: Default::default(),
            key: Default::default(),
            level: Default::default(),
            message: Default::default(),
            nextduedate: Default::default(),
            productname: Default::default(),
            regdate: Default::default(),
            serverid: Default::default(),
            signature: Default::default(),
            sockets: Default::default(),
            url: Default::default(),
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
    pub fn new(key: String) -> Self {
        Self {
            key,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[doc = "Proxmox VE subscription key"]
    #[doc = ""]
    pub key: String,
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
