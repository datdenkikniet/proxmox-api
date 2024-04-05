pub mod apiversion;
pub mod join;
pub mod nodes;
pub mod qdevice;
pub mod totem;
pub struct ConfigClient<T> {
    client: T,
    path: String,
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/config"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PostParams {
    pub fn new(clustername: String) -> Self {
        Self {
            clustername,
            link_n: Default::default(),
            nodeid: Default::default(),
            votes: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "The name of the cluster."]
    pub clustername: String,
    #[serde(rename = "link[n]")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)"]
    pub link_n: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node id for this node."]
    pub nodeid: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of votes for this node."]
    pub votes: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Generate new cluster configuration. If no links given, default to local IP address as link0."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn apiversion(&self) -> apiversion::ApiversionClient<T> {
        apiversion::ApiversionClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn nodes(&self) -> nodes::NodesClient<T> {
        nodes::NodesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn join(&self) -> join::JoinClient<T> {
        join::JoinClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn totem(&self) -> totem::TotemClient<T> {
        totem::TotemClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn qdevice(&self) -> qdevice::QdeviceClient<T> {
        qdevice::QdeviceClient::<T>::new(self.client.clone(), &self.path)
    }
}
