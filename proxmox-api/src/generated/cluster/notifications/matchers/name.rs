#[derive(Debug, Clone)]
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
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Remove matcher"]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Return a specific matcher"]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update existing matcher"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl GetOutput {
    pub fn new(name: String) -> Self {
        Self {
            name,
            comment: Default::default(),
            digest: Default::default(),
            disable: Default::default(),
            invert_match: Default::default(),
            match_calendar: Default::default(),
            match_field: Default::default(),
            match_severity: Default::default(),
            mode: Default::default(),
            target: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this matcher"]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(rename = "invert-match")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Invert match of the whole matcher"]
    #[doc = ""]
    pub invert_match: Option<bool>,
    #[serde(rename = "match-calendar")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Match notification timestamp"]
    #[doc = ""]
    pub match_calendar: Vec<String>,
    #[serde(rename = "match-field")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Metadata fields to match (regex or exact match). Must be in the form (regex|exact):\\\\<field\\\\>=\\\\<value\\\\>"]
    #[doc = ""]
    pub match_field: Vec<String>,
    #[serde(rename = "match-severity")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Notification severities to match"]
    #[doc = ""]
    pub match_severity: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Choose between 'all' and 'any' for when multiple properties are specified"]
    #[doc = ""]
    pub mode: Option<Mode>,
    #[doc = "Name of the matcher."]
    #[doc = ""]
    pub name: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Targets to notify on match"]
    #[doc = ""]
    pub target: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this matcher"]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(rename = "invert-match")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Invert match of the whole matcher"]
    #[doc = ""]
    pub invert_match: Option<bool>,
    #[serde(rename = "match-calendar")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Match notification timestamp"]
    #[doc = ""]
    pub match_calendar: Vec<String>,
    #[serde(rename = "match-field")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Metadata fields to match (regex or exact match). Must be in the form (regex|exact):\\\\<field\\\\>=\\\\<value\\\\>"]
    #[doc = ""]
    pub match_field: Vec<String>,
    #[serde(rename = "match-severity")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Notification severities to match"]
    #[doc = ""]
    pub match_severity: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Choose between 'all' and 'any' for when multiple properties are specified"]
    #[doc = ""]
    pub mode: Option<Mode>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Targets to notify on match"]
    #[doc = ""]
    pub target: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Choose between 'all' and 'any' for when multiple properties are specified"]
#[doc = ""]
pub enum Mode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any")]
    Any,
}
impl TryFrom<&str> for Mode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "all" => Ok(Self::All),
            "any" => Ok(Self::Any),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Mode {
    fn default() -> Self {
        Self::All
    }
}
