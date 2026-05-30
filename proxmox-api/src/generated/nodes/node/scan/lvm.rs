#[derive(Debug, Clone)]
pub struct LvmClient<T> {
    client: T,
    path: String,
}
impl<T> LvmClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/lvm"),
        }
    }
}
impl<T> LvmClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List local LVM volume groups."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/storage\", [\"Datastore.Allocate\"])"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(vg: String) -> Self {
        Self {
            vg,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The LVM logical volume group name."]
    #[doc = ""]
    pub vg: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
