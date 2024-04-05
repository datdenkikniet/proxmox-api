pub mod in_name;
pub mod lv_info;
pub mod metadata;
pub mod out;
pub mod scrub;
pub struct OsdidClient<T> {
    client: T,
    path: String,
}
impl<T> OsdidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, osdid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, osdid),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If set, we remove partition table entries."]
    pub cleanup: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> OsdidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Destroy OSD"]
    pub fn delete(&self, params: DeleteParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
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
impl<T> OsdidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "OSD index."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> OsdidClient<T>
where
    T: crate::client::Client,
{
    pub fn metadata(&self) -> metadata::MetadataClient<T> {
        metadata::MetadataClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> OsdidClient<T>
where
    T: crate::client::Client,
{
    pub fn lv_info(&self) -> lv_info::LvInfoClient<T> {
        lv_info::LvInfoClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> OsdidClient<T>
where
    T: crate::client::Client,
{
    pub fn in_name(&self) -> in_name::InNameClient<T> {
        in_name::InNameClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> OsdidClient<T>
where
    T: crate::client::Client,
{
    pub fn out(&self) -> out::OutClient<T> {
        out::OutClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> OsdidClient<T>
where
    T: crate::client::Client,
{
    pub fn scrub(&self) -> scrub::ScrubClient<T> {
        scrub::ScrubClient::<T>::new(self.client.clone(), &self.path)
    }
}
