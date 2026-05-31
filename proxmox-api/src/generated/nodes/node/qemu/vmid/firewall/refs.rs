#[derive(Debug, Clone)]
pub struct RefsClient<T> {
    client: T,
    path: String,
}
impl<T> RefsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/refs"),
        }
    }
}
impl<T> RefsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Lists possible IPSet/Alias reference which are allowed in source/dest properties."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/vms/{vmid}\", [\"VM.Audit\"])"]
    pub async fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &params).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(name: String, reference: String, scope: String, ty: Type) -> Self {
        Self {
            name,
            reference,
            scope,
            ty,
            comment: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub name: String,
    #[serde(rename = "ref")]
    pub reference: String,
    pub scope: String,
    #[serde(rename = "type")]
    pub ty: Type,
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
    #[doc = "Only list references of specified type."]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Only list references of specified type."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "alias")]
    Alias,
    #[serde(rename = "ipset")]
    Ipset,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "alias" => Ok(Self::Alias),
            "ipset" => Ok(Self::Ipset),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
