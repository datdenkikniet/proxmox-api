pub mod group;
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
    #[doc = "List security groups."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &GroupsClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
    }
}
impl<T> GroupsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create new security group."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, (), T::Error> for &GroupsClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<(), T::Error> {
        self.post(params)
    }
}
impl GetOutputItems {
    pub fn new(digest: String, group: String) -> Self {
        Self {
            digest,
            group,
            comment: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: String,
    #[doc = "Security Group name."]
    #[doc = ""]
    pub group: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(group: String) -> Self {
        Self {
            group,
            comment: Default::default(),
            digest: Default::default(),
            rename: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[doc = "Security Group name."]
    #[doc = ""]
    pub group: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Rename/update an existing security group. You can set 'rename' to the same value as 'name' to update the 'comment' of an existing group."]
    #[doc = ""]
    pub rename: Option<String>,
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
    pub fn group(&self, group: &str) -> group::GroupClient<T> {
        group::GroupClient::<T>::new(self.client.clone(), &self.path, group)
    }
}
