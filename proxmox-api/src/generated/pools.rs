pub mod poolid;
pub struct PoolsClient<T> {
    client: T,
    path: String,
}
impl<T> PoolsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "/pools".to_string(),
        }
    }
}
impl<T> PoolsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete pool."]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
impl<T> PoolsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List pools or get pool configuration."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> PoolsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create new pool."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> PoolsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update pool."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl DeleteParams {
    pub fn new(poolid: String) -> Self {
        Self {
            poolid,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct DeleteParams {
    pub poolid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(poolid: String) -> Self {
        Self {
            poolid,
            comment: Default::default(),
            members: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub members: Vec<MembersGetOutputItemsMembersItems>,
    pub poolid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub poolid: Option<String>,
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
impl MembersGetOutputItemsMembersItems {
    pub fn new(id: String, node: String, ty: Type) -> Self {
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
pub struct MembersGetOutputItemsMembersItems {
    pub id: String,
    pub node: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage: Option<String>,
    #[serde(rename = "type")]
    pub ty: Type,
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
impl PostParams {
    pub fn new(poolid: String) -> Self {
        Self {
            poolid,
            comment: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    pub poolid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PutParams {
    pub fn new(poolid: String) -> Self {
        Self {
            poolid,
            allow_move: Default::default(),
            comment: Default::default(),
            delete: Default::default(),
            storage: Default::default(),
            vms: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
    pub poolid: String,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "lxc")]
    Lxc,
    #[serde(rename = "openvz")]
    Openvz,
    #[serde(rename = "qemu")]
    Qemu,
    #[serde(rename = "storage")]
    Storage,
}
impl<T> PoolsClient<T>
where
    T: crate::client::Client,
{
    pub fn poolid(&self, poolid: &str) -> poolid::PoolidClient<T> {
        poolid::PoolidClient::<T>::new(self.client.clone(), &self.path, poolid)
    }
}
