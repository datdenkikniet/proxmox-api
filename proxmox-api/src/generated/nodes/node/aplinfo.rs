#[derive(Debug, Clone)]
pub struct AplinfoClient<T> {
    client: T,
    path: String,
}
impl<T> AplinfoClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/aplinfo"),
        }
    }
}
impl<T> AplinfoClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get list of appliances."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> AplinfoClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Download appliance templates."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
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
impl PostParams {
    pub fn new(storage: String, template: String) -> Self {
        Self {
            storage,
            template,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "The storage where the template will be stored"]
    #[doc = ""]
    pub storage: String,
    #[doc = "The template which will downloaded"]
    #[doc = ""]
    pub template: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
