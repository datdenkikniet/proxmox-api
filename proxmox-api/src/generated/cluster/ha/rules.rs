pub mod rule;
#[derive(Debug, Clone)]
pub struct RulesClient<T> {
    client: T,
    path: String,
}
impl<T> RulesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/rules"),
        }
    }
}
impl<T> RulesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get HA rules."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> RulesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create HA rule."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(rule: String) -> Self {
        Self {
            rule,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub rule: String,
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
    #[doc = "Limit the returned list to rules affecting the specified resource."]
    #[doc = ""]
    pub resource: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Limit the returned list to the specified rule type."]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(resources: String, rule: String, ty: Type) -> Self {
        Self {
            resources,
            rule,
            ty,
            affinity: ::std::default::Default::default(),
            comment: ::std::default::Default::default(),
            disable: ::std::default::Default::default(),
            nodes: ::std::default::Default::default(),
            strict: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Describes whether the HA resources are supposed to be kept on the same node ('positive'), or are supposed to be kept on separate nodes ('negative')."]
    #[doc = ""]
    pub affinity: Option<Affinity>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HA rule description."]
    #[doc = ""]
    pub comment: Option<CommentStr>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Whether the HA rule is disabled."]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of cluster node names with optional priority."]
    #[doc = ""]
    #[doc = "List of cluster node members, where a priority can be given to each node. A resource will run on the available nodes with the highest priority. If there are more nodes in the highest priority class, the resources will get distributed to those nodes. The priorities have a relative meaning only. The higher the number, the higher the priority."]
    #[doc = ""]
    pub nodes: Option<String>,
    #[doc = "List of HA resource IDs. This consists of a list of resource types followed by a resource specific name separated with a colon (example: vm:100,ct:101)."]
    #[doc = ""]
    pub resources: String,
    #[doc = "HA rule identifier."]
    #[doc = ""]
    pub rule: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Describes whether the node affinity rule is strict or non-strict."]
    #[doc = ""]
    #[doc = "Describes whether the node affinity rule is strict or non-strict."]
    #[doc = ""]
    #[doc = "A non-strict node affinity rule makes resources prefer to be on the defined nodes."]
    #[doc = ""]
    #[doc = "If none of the defined nodes are available, the resource may run on any other node."]
    #[doc = ""]
    #[doc = "A strict node affinity rule makes resources be restricted to the defined nodes. If"]
    #[doc = ""]
    #[doc = "none of the defined nodes are available, the resource will be stopped."]
    #[doc = ""]
    pub strict: Option<bool>,
    #[serde(rename = "type")]
    #[doc = "HA rule type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Describes whether the HA resources are supposed to be kept on the same node ('positive'), or are supposed to be kept on separate nodes ('negative')."]
#[doc = ""]
pub enum Affinity {
    #[serde(rename = "negative")]
    Negative,
    #[serde(rename = "positive")]
    Positive,
}
impl TryFrom<&str> for Affinity {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "negative" => Ok(Self::Negative),
            "positive" => Ok(Self::Positive),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Limit the returned list to the specified rule type."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "node-affinity")]
    NodeAffinity,
    #[serde(rename = "resource-affinity")]
    ResourceAffinity,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "node-affinity" => Ok(Self::NodeAffinity),
            "resource-affinity" => Ok(Self::ResourceAffinity),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CommentStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for CommentStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(4096usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 4096";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for CommentStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for CommentStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CommentStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
impl<T> RulesClient<T>
where
    T: crate::client::Client,
{
    pub fn rule(&self, rule: &str) -> rule::RuleClient<T> {
        rule::RuleClient::<T>::new(self.client.clone(), &self.path, rule)
    }
}
