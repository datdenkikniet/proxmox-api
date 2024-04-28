#[derive(Debug, Clone)]
pub struct FlagClient<T> {
    client: T,
    path: String,
}
impl<T> FlagClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, flag: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, flag),
        }
    }
}
impl<T> FlagClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the status of a specific ceph flag."]
    #[doc = ""]
    pub fn get(&self) -> Result<bool, T::Error> {
        let path = self.path.to_string();
        Ok(self.client.get::<_, crate::types::Bool>(&path, &())?.get())
    }
}
impl<T> FlagClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set or clear (unset) a specific ceph flag"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        match self.client.put(&path, &params) {
            Ok(o) => Ok(o),
            Err(e) if crate::client::Error::is_empty_data(&e) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
impl PutParams {
    pub fn new(value: bool) -> Self {
        Self {
            value,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "The new value of the flag"]
    #[doc = ""]
    pub value: bool,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
