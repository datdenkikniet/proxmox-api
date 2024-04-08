pub struct IncludedVolumesClient<T> {
    client: T,
    path: String,
}
impl<T> IncludedVolumesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/included_volumes"),
        }
    }
}
impl<T> IncludedVolumesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns included guests and the backup status of their disks. Optimized to be used in ExtJS tree views."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl ChildrenGetOutputChildrenItems {
    pub fn new(id: u64, ty: Type) -> Self {
        Self {
            id,
            ty,
            children: Default::default(),
            name: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct ChildrenGetOutputChildrenItems {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "The volumes of the guest with the information if they will be included in backups."]
    #[doc = ""]
    pub children: Vec<ChildrenGetOutputChildrenItemsChildrenItems>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "VMID of the guest."]
    #[doc = ""]
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Name of the guest"]
    #[doc = ""]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Type of the guest, VM, CT or unknown for removed but not purged guests."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl ChildrenGetOutputChildrenItemsChildrenItems {
    pub fn new(id: String, included: bool, name: String, reason: String) -> Self {
        Self {
            id,
            included,
            name,
            reason,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct ChildrenGetOutputChildrenItemsChildrenItems {
    #[doc = "Configuration key of the volume."]
    #[doc = ""]
    pub id: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "Whether the volume is included in the backup or not."]
    #[doc = ""]
    pub included: bool,
    #[doc = "Name of the volume."]
    #[doc = ""]
    pub name: String,
    #[doc = "The reason why the volume is included (or excluded)."]
    #[doc = ""]
    pub reason: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(children: Vec<ChildrenGetOutputChildrenItems>) -> Self {
        Self {
            children,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub children: Vec<ChildrenGetOutputChildrenItems>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Type of the guest, VM, CT or unknown for removed but not purged guests."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "lxc")]
    Lxc,
    #[serde(rename = "qemu")]
    Qemu,
    #[serde(rename = "unknown")]
    Unknown,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "lxc" => Ok(Self::Lxc),
            "qemu" => Ok(Self::Qemu),
            "unknown" => Ok(Self::Unknown),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
