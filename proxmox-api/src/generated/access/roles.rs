pub mod roleid;
pub struct RolesClient<T> {
    client: T,
    path: String,
}
impl<T> RolesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/roles"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a RolesClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> RolesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Role index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> RolesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create new role."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(roleid: String) -> Self {
        Self {
            roleid,
            privs: Default::default(),
            special: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub privs: Option<String>,
    pub roleid: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub special: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(roleid: String) -> Self {
        Self {
            roleid,
            privs: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub privs: Option<String>,
    pub roleid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> RolesClient<T>
where
    T: crate::client::Client,
{
    pub fn roleid(&self, roleid: &str) -> roleid::RoleidClient<T> {
        roleid::RoleidClient::<T>::new(self.client.clone(), &self.path, roleid)
    }
}
