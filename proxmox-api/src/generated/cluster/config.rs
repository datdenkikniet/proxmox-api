pub mod apiversion;
pub mod join;
pub mod nodes;
pub mod qdevice;
pub mod totem;
#[derive(Debug, Clone)]
pub struct ConfigClient<T> {
    client: T,
    path: String,
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/config"),
        }
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Generate new cluster configuration. If no links given, default to local IP address as link0."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(clustername: ClusternameStr) -> Self {
        Self {
            clustername,
            links: Default::default(),
            nodeid: Default::default(),
            votes: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "The name of the cluster."]
    #[doc = ""]
    pub clustername: ClusternameStr,
    #[serde(rename = "link[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedLinks, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedLinks, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)"]
    #[doc = ""]
    pub links: ::std::collections::HashMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node id for this node."]
    #[doc = ""]
    pub nodeid: Option<NodeidInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of votes for this node."]
    #[doc = ""]
    pub votes: Option<VotesInt>,
    #[serde(
        flatten,
        deserialize_with = "crate::types::multi::deserialize_additional_data::<'_, PostParams, _, _>"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl crate::types::multi::Test for PostParams {
    fn test_fn() -> fn(&str) -> bool {
        fn the_test(input: &str) -> bool {
            let array = [
                <NumberedLinks as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
            ];
            array.iter().any(|f| f(input))
        }
        the_test as _
    }
}
#[derive(Default)]
struct NumberedLinks;
impl crate::types::multi::NumberedItems for NumberedLinks {
    type Item = String;
    const PREFIX: &'static str = "link";
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NodeidInt(i128);
impl crate::types::bounded_integer::BoundedInteger for NodeidInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 1";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for NodeidInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for NodeidInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for NodeidInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VotesInt(i128);
impl crate::types::bounded_integer::BoundedInteger for VotesInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 1";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for VotesInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for VotesInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VotesInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ClusternameStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for ClusternameStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(15usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 15";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for ClusternameStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for ClusternameStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ClusternameStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn apiversion(&self) -> apiversion::ApiversionClient<T> {
        apiversion::ApiversionClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn nodes(&self) -> nodes::NodesClient<T> {
        nodes::NodesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn join(&self) -> join::JoinClient<T> {
        join::JoinClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn totem(&self) -> totem::TotemClient<T> {
        totem::TotemClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn qdevice(&self) -> qdevice::QdeviceClient<T> {
        qdevice::QdeviceClient::<T>::new(self.client.clone(), &self.path)
    }
}
