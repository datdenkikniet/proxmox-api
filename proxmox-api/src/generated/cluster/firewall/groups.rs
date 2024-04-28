pub mod group;
#[derive(Debug, Clone)]
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
impl<T> GroupsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List security groups."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> GroupsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create new security group."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        match self.client.post(&path, &params) {
            Ok(o) => Ok(o),
            Err(e) if crate::client::Error::is_empty_data(&e) => Ok(()),
            Err(e) => Err(e),
        }
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
