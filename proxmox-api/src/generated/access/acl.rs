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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "user")]
    User,
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
    pub path: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow to propagate (inherit) permissions."]
    pub propagate: Option<bool>,
    pub roleid: String,
    #[serde(rename = "type")]
    pub ty: Type,
    pub ugid: String,
}
impl<T> AclClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get Access Control List (ACLs)."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
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
    pub delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of groups."]
    pub groups: Option<String>,
    #[doc = "Access control path"]
    pub path: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow to propagate (inherit) permissions."]
    pub propagate: Option<bool>,
    #[doc = "List of roles."]
    pub roles: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of API tokens."]
    pub tokens: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of users."]
    pub users: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> AclClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update Access Control List (add or remove permissions)."]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
