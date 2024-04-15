pub mod groupid;
pub struct GroupsClient<T> {
    client: T,
    path: String,
}
impl<T> GroupsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/groups"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a GroupsClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> GroupsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Group index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> GroupsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create new group."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(groupid: String) -> Self {
        Self {
            groupid,
            comment: Default::default(),
            users: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub groupid: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "list of users which form this group"]
    #[doc = ""]
    pub users: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(groupid: String) -> Self {
        Self {
            groupid,
            comment: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub groupid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> GroupsClient<T>
where
    T: crate::client::Client,
{
    pub fn groupid(&self, groupid: &str) -> groupid::GroupidClient<T> {
        groupid::GroupidClient::<T>::new(self.client.clone(), &self.path, groupid)
    }
}
