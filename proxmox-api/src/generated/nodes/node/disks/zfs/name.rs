pub struct NameClient<T> {
    client: T,
    path: String,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a NameClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Destroy a ZFS pool."]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.delete(&path, &params)
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get details about a zpool."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl ChildrenGetOutputChildrenItems {
    pub fn new(msg: String, name: String) -> Self {
        Self {
            msg,
            name,
            cksum: Default::default(),
            read: Default::default(),
            state: Default::default(),
            write: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct ChildrenGetOutputChildrenItems {
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub cksum: Option<f64>,
    #[doc = "An optional message about the vdev."]
    #[doc = ""]
    pub msg: String,
    #[doc = "The name of the vdev or section."]
    #[doc = ""]
    pub name: String,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub read: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The state of the vdev."]
    #[doc = ""]
    pub state: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub write: Option<f64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(rename = "cleanup-config")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Marks associated storage(s) as not available on this node anymore or removes them from the configuration (if configured for this node only)."]
    #[doc = ""]
    pub cleanup_config: Option<bool>,
    #[serde(rename = "cleanup-disks")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Also wipe disks so they can be repurposed afterwards."]
    #[doc = ""]
    pub cleanup_disks: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(
        children: Vec<ChildrenGetOutputChildrenItems>,
        errors: String,
        name: String,
        state: String,
    ) -> Self {
        Self {
            children,
            errors,
            name,
            state,
            action: Default::default(),
            scan: Default::default(),
            status: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Information about the recommended action to fix the state."]
    #[doc = ""]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "The pool configuration information, including the vdevs for each section (e.g. spares, cache), may be nested."]
    #[doc = ""]
    pub children: Vec<ChildrenGetOutputChildrenItems>,
    #[doc = "Information about the errors on the zpool."]
    #[doc = ""]
    pub errors: String,
    #[doc = "The name of the zpool."]
    #[doc = ""]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Information about the last/current scrub."]
    #[doc = ""]
    pub scan: Option<String>,
    #[doc = "The state of the zpool."]
    #[doc = ""]
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Information about the state of the zpool."]
    #[doc = ""]
    pub status: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
