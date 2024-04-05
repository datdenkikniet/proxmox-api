pub mod name;
pub struct TargetsClient<T> {
    client: T,
    path: String,
}
impl<T> TargetsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/targets"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Origin {
    #[serde(rename = "builtin")]
    Builtin,
    #[serde(rename = "modified-builtin")]
    ModifiedBuiltin,
    #[serde(rename = "user-created")]
    UserCreated,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "gotify")]
    Gotify,
    #[serde(rename = "sendmail")]
    Sendmail,
}
impl GetOutputItems {
    pub fn new(name: String, origin: Origin, ty: Type) -> Self {
        Self {
            name,
            origin,
            ty,
            comment: Default::default(),
            disable: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Show if this target is disabled"]
    pub disable: Option<bool>,
    #[doc = "Name of the target."]
    pub name: String,
    #[doc = "Show if this entry was created by a user or was built-in"]
    pub origin: Origin,
    #[serde(rename = "type")]
    #[doc = "Type of the target."]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> TargetsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns a list of all entities that can be used as notification targets."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> TargetsClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
