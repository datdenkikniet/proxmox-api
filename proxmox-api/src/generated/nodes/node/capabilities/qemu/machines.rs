#[derive(Debug, Clone)]
pub struct MachinesClient<T> {
    client: T,
    path: String,
}
impl<T> MachinesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/machines"),
        }
    }
}
impl<T> MachinesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get available QEMU/KVM machine types."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(id: String, ty: Type, version: String) -> Self {
        Self {
            id,
            ty,
            version,
            changes: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Notable changes of a version, currently only set for +pveX versions."]
    #[doc = ""]
    pub changes: Option<String>,
    #[doc = "Full name of machine type and version."]
    #[doc = ""]
    pub id: String,
    #[serde(rename = "type")]
    #[doc = "The machine type."]
    #[doc = ""]
    pub ty: Type,
    #[doc = "The machine version."]
    #[doc = ""]
    pub version: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The machine type."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "i440fx")]
    I440fx,
    #[serde(rename = "q35")]
    Q35,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "i440fx" => Ok(Self::I440fx),
            "q35" => Ok(Self::Q35),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
