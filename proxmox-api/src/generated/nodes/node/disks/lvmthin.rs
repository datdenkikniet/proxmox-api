pub mod name;
#[derive(Debug, Clone)]
pub struct LvmthinClient<T> {
    client: T,
    path: String,
}
impl<T> LvmthinClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/lvmthin"),
        }
    }
}
impl<T> LvmthinClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List LVM thinpools"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> LvmthinClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create an LVM thinpool"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(
        lv: String,
        lv_size: i64,
        metadata_size: i64,
        metadata_used: i64,
        used: i64,
        vg: String,
    ) -> Self {
        Self {
            lv,
            lv_size,
            metadata_size,
            metadata_used,
            used,
            vg,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The name of the thinpool."]
    #[doc = ""]
    pub lv: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The size of the thinpool in bytes."]
    #[doc = ""]
    pub lv_size: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The size of the metadata lv in bytes."]
    #[doc = ""]
    pub metadata_size: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The used bytes of the metadata lv."]
    #[doc = ""]
    pub metadata_used: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The used bytes of the thinpool."]
    #[doc = ""]
    pub used: i64,
    #[doc = "The associated volume group."]
    #[doc = ""]
    pub vg: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(device: String, name: String) -> Self {
        Self {
            device,
            name,
            add_storage: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure storage using the thinpool."]
    #[doc = ""]
    pub add_storage: Option<bool>,
    #[doc = "The block device you want to create the thinpool on."]
    #[doc = ""]
    pub device: String,
    #[doc = "The storage identifier."]
    #[doc = ""]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> LvmthinClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
