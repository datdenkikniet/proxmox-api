pub mod name;
pub struct LvmClient<T> {
    client: T,
    path: String,
}
impl<T> LvmClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/lvm"),
        }
    }
}
impl ChildrenGetOutputChildrenItemsChildrenItems {
    pub fn new(free: u64, leaf: bool, name: String, size: u64) -> Self {
        Self {
            free,
            leaf,
            name,
            size,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct ChildrenGetOutputChildrenItemsChildrenItems {
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    #[doc = "The free bytes in the physical volume"]
    pub free: u64,
    #[serde(
        serialize_with = "crate::serialize_bool",
        deserialize_with = "crate::deserialize_bool"
    )]
    pub leaf: bool,
    #[doc = "The name of the physical volume"]
    pub name: String,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    #[doc = "The size of the physical volume in bytes"]
    pub size: u64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl ChildrenGetOutputChildrenItems {
    pub fn new(free: u64, leaf: bool, name: String, size: u64) -> Self {
        Self {
            free,
            leaf,
            name,
            size,
            children: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct ChildrenGetOutputChildrenItems {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "The underlying physical volumes"]
    pub children: Vec<ChildrenGetOutputChildrenItemsChildrenItems>,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    #[doc = "The free bytes in the volume group"]
    pub free: u64,
    #[serde(
        serialize_with = "crate::serialize_bool",
        deserialize_with = "crate::deserialize_bool"
    )]
    pub leaf: bool,
    #[doc = "The name of the volume group"]
    pub name: String,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    #[doc = "The size of the volume group in bytes"]
    pub size: u64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(children: Vec<ChildrenGetOutputChildrenItems>, leaf: bool) -> Self {
        Self {
            children,
            leaf,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub children: Vec<ChildrenGetOutputChildrenItems>,
    #[serde(
        serialize_with = "crate::serialize_bool",
        deserialize_with = "crate::deserialize_bool"
    )]
    pub leaf: bool,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> LvmClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List LVM Volume Groups"]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PostParams {
    pub fn new(device: String, name: String) -> Self {
        Self {
            device,
            name,
            add_storage: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure storage using the Volume Group"]
    pub add_storage: Option<bool>,
    #[doc = "The block device you want to create the volume group on"]
    pub device: String,
    #[doc = "The storage identifier."]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> LvmClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create an LVM Volume Group"]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> LvmClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
