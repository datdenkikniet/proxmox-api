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
    pub fn get(&self) -> Result<bool, T::Error> {
        let path = self.path.to_string();
        Ok(self.client.get::<_, crate::types::Bool>(&path, &())?.get())
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
    pub value: bool,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> FlagClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set or clear (unset) a specific ceph flag"]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
