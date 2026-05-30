#[derive(Debug, Clone)]
pub struct ListClient<T> {
    client: T,
    path: String,
}
impl<T> ListClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/list"),
        }
    }
}
impl<T> ListClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List files and directories for single file restore under the given path."]
    #[doc = ""]
    #[doc = "You need read access for the volume."]
    pub async fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &params).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(filepath: String, leaf: bool, text: String, ty: String) -> Self {
        Self {
            filepath,
            leaf,
            text,
            ty,
            mtime: ::std::default::Default::default(),
            size: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "base64 path of the current entry"]
    #[doc = ""]
    pub filepath: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "If this entry is a leaf in the directory graph."]
    #[doc = ""]
    pub leaf: bool,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Entry last-modified time (unix timestamp)."]
    #[doc = ""]
    pub mtime: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Entry file size."]
    #[doc = ""]
    pub size: Option<i64>,
    #[doc = "Entry display text."]
    #[doc = ""]
    pub text: String,
    #[serde(rename = "type")]
    #[doc = "Entry type."]
    #[doc = ""]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(filepath: String, volume: String) -> Self {
        Self {
            filepath,
            volume,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "base64-path to the directory or file being listed, or \"/\"."]
    #[doc = ""]
    pub filepath: String,
    #[doc = "Backup volume ID or name. Currently only PBS snapshots are supported."]
    #[doc = ""]
    pub volume: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
