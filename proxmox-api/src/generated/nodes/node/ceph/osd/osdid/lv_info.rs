pub struct LvInfoClient<T> {
    client: T,
    path: String,
}
impl<T> LvInfoClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/lv-info"),
        }
    }
}
impl<T> LvInfoClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get OSD volume details"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutput {
    pub fn new(
        creation_time: String,
        lv_name: String,
        lv_path: String,
        lv_size: u64,
        lv_uuid: String,
        vg_name: String,
    ) -> Self {
        Self {
            creation_time,
            lv_name,
            lv_path,
            lv_size,
            lv_uuid,
            vg_name,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[doc = "Creation time as reported by `lvs`."]
    #[doc = ""]
    pub creation_time: String,
    #[doc = "Name of the logical volume (LV)."]
    #[doc = ""]
    pub lv_name: String,
    #[doc = "Path to the logical volume (LV)."]
    #[doc = ""]
    pub lv_path: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Size of the logical volume (LV)."]
    #[doc = ""]
    pub lv_size: u64,
    #[doc = "UUID of the logical volume (LV)."]
    #[doc = ""]
    pub lv_uuid: String,
    #[doc = "Name of the volume group (VG)."]
    #[doc = ""]
    pub vg_name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSD device type"]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "OSD device type"]
#[doc = ""]
pub enum Type {
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "db")]
    Db,
    #[serde(rename = "wal")]
    Wal,
}
impl Default for Type {
    fn default() -> Self {
        Self::Block
    }
}
