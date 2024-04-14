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
impl<T> TargetsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns a list of all entities that can be used as notification targets."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
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
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Show if this target is disabled"]
    #[doc = ""]
    pub disable: Option<bool>,
    #[doc = "Name of the target."]
    #[doc = ""]
    pub name: String,
    #[doc = "Show if this entry was created by a user or was built-in"]
    #[doc = ""]
    pub origin: Origin,
    #[serde(rename = "type")]
    #[doc = "Type of the target."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Show if this entry was created by a user or was built-in"]
#[doc = ""]
pub enum Origin {
    #[serde(rename = "builtin")]
    Builtin,
    #[serde(rename = "modified-builtin")]
    ModifiedBuiltin,
    #[serde(rename = "user-created")]
    UserCreated,
}
impl TryFrom<&str> for Origin {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "builtin" => Ok(Self::Builtin),
            "modified-builtin" => Ok(Self::ModifiedBuiltin),
            "user-created" => Ok(Self::UserCreated),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Type of the target."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "gotify")]
    Gotify,
    #[serde(rename = "sendmail")]
    Sendmail,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "gotify" => Ok(Self::Gotify),
            "sendmail" => Ok(Self::Sendmail),
            v => Err(format!("Unknown variant {v}")),
        }
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
