pub struct PoolidClient<T> {
    client: T,
    path: String,
}
impl<T> PoolidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, poolid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, poolid),
        }
    }
}
impl<T> PoolidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete pool (deprecated, no support for nested pools, use 'DELETE /pools/?poolid={poolid}')."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> PoolidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get pool configuration (deprecated, no support for nested pools, use 'GET /pools/?poolid={poolid}')."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> PoolidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update pool data (deprecated, no support for nested pools - use 'PUT /pools/?poolid={poolid}' instead)."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl GetOutput {
    pub fn new(members: Vec<MembersGetOutputMembersItems>) -> Self {
        Self {
            members,
            comment: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub members: Vec<MembersGetOutputMembersItems>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl MembersGetOutputMembersItems {
    pub fn new(id: String, node: String, ty: Type2) -> Self {
        Self {
            id,
            node,
            ty,
            storage: Default::default(),
            vmid: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct MembersGetOutputMembersItems {
    pub id: String,
    pub node: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[serde(rename = "type")]
    pub ty: Type2,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vmid: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(rename = "allow-move")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow adding a guest even if already in another pool. The guest will be removed from its current pool and added to this one."]
    #[doc = ""]
    pub allow_move: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Remove the passed VMIDs and/or storage IDs instead of adding them."]
    #[doc = ""]
    pub delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of storage IDs to add or remove from this pool."]
    #[doc = ""]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of guest VMIDs to add or remove from this pool."]
    #[doc = ""]
    pub vms: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
pub enum Type {
    #[serde(rename = "lxc")]
    Lxc,
    #[serde(rename = "qemu")]
    Qemu,
    #[serde(rename = "storage")]
    Storage,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "lxc" => Ok(Self::Lxc),
            "qemu" => Ok(Self::Qemu),
            "storage" => Ok(Self::Storage),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
pub enum Type2 {
    #[serde(rename = "lxc")]
    Lxc,
    #[serde(rename = "openvz")]
    Openvz,
    #[serde(rename = "qemu")]
    Qemu,
    #[serde(rename = "storage")]
    Storage,
}
impl TryFrom<&str> for Type2 {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "lxc" => Ok(Self::Lxc),
            "openvz" => Ok(Self::Openvz),
            "qemu" => Ok(Self::Qemu),
            "storage" => Ok(Self::Storage),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
