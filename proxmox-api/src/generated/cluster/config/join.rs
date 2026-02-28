#[derive(Debug, Clone)]
pub struct JoinClient<T> {
    client: T,
    path: String,
}
impl<T> JoinClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/join"),
        }
    }
}
impl<T> JoinClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get information needed to join this cluster over the connected node."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> JoinClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Joins this node into an existing cluster. If no links are given, default to IP resolved by node's hostname on single link (fallback fails for clusters with multiple links)."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutput {
    pub fn new(
        config_digest: String,
        nodelist: Vec<NodelistGetOutputNodelistItems>,
        preferred_node: String,
        totem: TotemGetOutputTotem,
    ) -> Self {
        Self {
            config_digest,
            nodelist,
            preferred_node,
            totem,
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    pub config_digest: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub nodelist: Vec<NodelistGetOutputNodelistItems>,
    #[doc = "The cluster node name."]
    #[doc = ""]
    pub preferred_node: String,
    pub totem: TotemGetOutputTotem,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The node for which the joinee gets the nodeinfo."]
    #[doc = ""]
    pub node: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl NodelistGetOutputNodelistItems {
    pub fn new(
        name: String,
        pve_addr: ::std::net::IpAddr,
        pve_fp: PveFpStr,
        quorum_votes: QuorumVotesInt,
    ) -> Self {
        Self {
            name,
            pve_addr,
            pve_fp,
            quorum_votes,
            nodeid: Default::default(),
            ring0_addr: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct NodelistGetOutputNodelistItems {
    #[doc = "The cluster node name."]
    #[doc = ""]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node id for this node."]
    #[doc = ""]
    pub nodeid: Option<NodeidInt>,
    pub pve_addr: ::std::net::IpAddr,
    #[doc = "Certificate SHA 256 fingerprint."]
    #[doc = ""]
    pub pve_fp: PveFpStr,
    pub quorum_votes: QuorumVotesInt,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)"]
    #[doc = ""]
    pub ring0_addr: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(fingerprint: FingerprintStr, hostname: String, password: PasswordStr) -> Self {
        Self {
            fingerprint,
            hostname,
            password,
            force: Default::default(),
            links: Default::default(),
            nodeid: Default::default(),
            votes: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Certificate SHA 256 fingerprint."]
    #[doc = ""]
    pub fingerprint: FingerprintStr,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Do not throw error if node already exists."]
    #[doc = ""]
    pub force: Option<bool>,
    #[doc = "Hostname (or IP) of an existing cluster member."]
    #[doc = ""]
    pub hostname: String,
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
    #[doc = "Superuser (root) password of peer node."]
    #[doc = ""]
    pub password: PasswordStr,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of votes for this node"]
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct TotemGetOutputTotem {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
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
pub struct QuorumVotesInt(i128);
impl crate::types::bounded_integer::BoundedInteger for QuorumVotesInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for QuorumVotesInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for QuorumVotesInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for QuorumVotesInt {
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
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
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
pub struct FingerprintStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for FingerprintStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("([A-Fa-f0-9]{2}:){31}[A-Fa-f0-9]{2}");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"([A-Fa-f0-9]{2}:){31}[A-Fa-f0-9]{2}\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for FingerprintStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for FingerprintStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for FingerprintStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PasswordStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for PasswordStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(128usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 128";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for PasswordStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for PasswordStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PasswordStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PveFpStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for PveFpStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("([A-Fa-f0-9]{2}:){31}[A-Fa-f0-9]{2}");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"([A-Fa-f0-9]{2}:){31}[A-Fa-f0-9]{2}\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for PveFpStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for PveFpStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PveFpStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
