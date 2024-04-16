pub struct ChallengeSchemaClient<T> {
    client: T,
    path: String,
}
impl<T> ChallengeSchemaClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/challenge-schema"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ChallengeSchemaClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ChallengeSchemaClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get schema of ACME challenge types."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &ChallengeSchemaClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
    }
}
impl GetOutputItems {
    pub fn new(id: String, name: String, schema: SchemaGetOutputItemsSchema, ty: String) -> Self {
        Self {
            id,
            name,
            schema,
            ty,
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub id: String,
    #[doc = "Human readable name, falls back to id"]
    #[doc = ""]
    pub name: String,
    pub schema: SchemaGetOutputItemsSchema,
    #[serde(rename = "type")]
    pub ty: String,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct SchemaGetOutputItemsSchema {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
