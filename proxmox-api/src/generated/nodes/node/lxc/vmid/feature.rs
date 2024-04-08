pub struct FeatureClient<T> {
    client: T,
    path: String,
}
impl<T> FeatureClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/feature"),
        }
    }
}
impl<T> FeatureClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Check if feature for virtual machine is available."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutput {
    pub fn new(hasfeature: bool) -> Self {
        Self {
            hasfeature,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(rename = "hasFeature")]
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub hasfeature: bool,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(feature: Feature) -> Self {
        Self {
            feature,
            snapname: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "Feature to check."]
    #[doc = ""]
    pub feature: Feature,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The name of the snapshot."]
    #[doc = ""]
    pub snapname: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Feature to check."]
#[doc = ""]
pub enum Feature {
    #[serde(rename = "clone")]
    Clone,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "snapshot")]
    Snapshot,
}
