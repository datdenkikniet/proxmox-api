pub mod snapname;
#[derive(Debug, Clone)]
pub struct SnapshotClient<T> {
    client: T,
    path: String,
}
impl<T> SnapshotClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/snapshot"),
        }
    }
}
impl<T> SnapshotClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List all snapshots."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> SnapshotClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Snapshot a container."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(description: String, name: String) -> Self {
        Self {
            description,
            name,
            parent: Default::default(),
            snaptime: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Snapshot description."]
    #[doc = ""]
    pub description: String,
    #[doc = "Snapshot identifier. Value 'current' identifies the current VM."]
    #[doc = ""]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Parent snapshot identifier."]
    #[doc = ""]
    pub parent: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Snapshot creation time"]
    #[doc = ""]
    pub snaptime: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(snapname: SnapnameStr) -> Self {
        Self {
            snapname,
            description: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A textual description or comment."]
    #[doc = ""]
    pub description: Option<String>,
    #[doc = "The name of the snapshot."]
    #[doc = ""]
    pub snapname: SnapnameStr,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SnapnameStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for SnapnameStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(40usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 40";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for SnapnameStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for SnapnameStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SnapnameStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
impl<T> SnapshotClient<T>
where
    T: crate::client::Client,
{
    pub fn snapname(&self, snapname: &str) -> snapname::SnapnameClient<T> {
        snapname::SnapnameClient::<T>::new(self.client.clone(), &self.path, snapname)
    }
}
