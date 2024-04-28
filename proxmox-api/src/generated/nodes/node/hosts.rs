#[derive(Debug, Clone)]
pub struct HostsClient<T> {
    client: T,
    path: String,
}
impl<T> HostsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/hosts"),
        }
    }
}
impl<T> HostsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the content of /etc/hosts."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> HostsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Write /etc/hosts."]
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
impl GetOutput {
    pub fn new(data: String) -> Self {
        Self {
            data,
            digest: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[doc = "The content of /etc/hosts."]
    #[doc = ""]
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(data: String) -> Self {
        Self {
            data,
            digest: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "The target content of /etc/hosts."]
    #[doc = ""]
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
