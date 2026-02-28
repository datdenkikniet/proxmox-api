#[derive(Debug, Clone)]
pub struct WipediskClient<T> {
    client: T,
    path: String,
}
impl<T> WipediskClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/wipedisk"),
        }
    }
}
impl<T> WipediskClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Wipe a disk or partition."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl PutParams {
    pub fn new(disk: DiskStr) -> Self {
        Self {
            disk,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[doc = "Block device name"]
    #[doc = ""]
    pub disk: DiskStr,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DiskStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DiskStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("^/dev/[a-zA-Z0-9\\/]+$");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"^/dev/[a-zA-Z0-9\\/]+$\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DiskStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DiskStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DiskStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
