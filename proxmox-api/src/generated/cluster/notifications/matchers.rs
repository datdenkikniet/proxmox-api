pub mod name;
pub struct MatchersClient<T> {
    client: T,
    path: String,
}
impl<T> MatchersClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/matchers"),
        }
    }
}
impl<T> MatchersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns a list of all matchers"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> MatchersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new matcher"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(name: String, origin: Origin) -> Self {
        Self {
            name,
            origin,
            comment: Default::default(),
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
    #[doc = "Show if this entry was created by a user or was built-in"]
    #[doc = ""]
    pub origin: Origin,
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
impl PostParams {
    pub fn new(name: String) -> Self {
        Self {
            name,
            comment: Default::default(),
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
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    #[doc = ""]
    pub comment: Option<String>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
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
impl<T> MatchersClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
