#[derive(Debug, Clone)]
pub struct AclClient<T> {
    client: T,
    path: String,
}
impl<T> AclClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/acl"),
        }
    }
}
impl<T> AclClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get Access Control List (ACLs)."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> AclClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update Access Control List (add or remove permissions)."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(path: String, roleid: String, ty: Type, ugid: String) -> Self {
        Self {
            path,
            roleid,
            ty,
            ugid,
            propagate: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Access control path"]
    #[doc = ""]
    pub path: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow to propagate (inherit) permissions."]
    #[doc = ""]
    pub propagate: Option<bool>,
    pub roleid: String,
    #[serde(rename = "type")]
    pub ty: Type,
    pub ugid: String,
}
impl PutParams {
    pub fn new(path: String, roles: String) -> Self {
        Self {
            path,
            roles,
            delete: Default::default(),
            groups: Default::default(),
            propagate: Default::default(),
            tokens: Default::default(),
            users: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Remove permissions (instead of adding it)."]
    #[doc = ""]
    pub delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of groups."]
    #[doc = ""]
    pub groups: Option<String>,
    #[doc = "Access control path"]
    #[doc = ""]
    pub path: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow to propagate (inherit) permissions."]
    #[doc = ""]
    pub propagate: Option<bool>,
    #[doc = "List of roles."]
    #[doc = ""]
    pub roles: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of API tokens."]
    #[doc = ""]
    pub tokens: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of users."]
    #[doc = ""]
    pub users: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
pub enum Type {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "user")]
    User,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "group" => Ok(Self::Group),
            "token" => Ok(Self::Token),
            "user" => Ok(Self::User),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
