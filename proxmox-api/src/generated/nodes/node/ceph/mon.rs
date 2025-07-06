pub mod monid;
#[derive(Debug, Clone)]
pub struct MonClient<T> {
    client: T,
    path: String,
}
impl<T> MonClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/mon"),
        }
    }
}
impl<T> MonClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get Ceph monitor list."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(name: String) -> Self {
        Self {
            name,
            addr: Default::default(),
            ceph_version: Default::default(),
            ceph_version_short: Default::default(),
            direxists: Default::default(),
            host: Default::default(),
            quorum: Default::default(),
            rank: Default::default(),
            service: Default::default(),
            state: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ceph_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ceph_version_short: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub direxists: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub host: Option<bool>,
    pub name: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub quorum: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rank: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub service: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> MonClient<T>
where
    T: crate::client::Client,
{
    pub fn monid(&self, monid: &str) -> monid::MonidClient<T> {
        monid::MonidClient::<T>::new(self.client.clone(), &self.path, monid)
    }
}
