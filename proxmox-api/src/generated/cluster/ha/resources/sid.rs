pub mod migrate;
pub mod relocate;
#[derive(Debug, Clone)]
pub struct SidClient<T> {
    client: T,
    path: String,
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, sid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, sid),
        }
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete resource configuration."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read resource configuration."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update resource configuration."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl GetOutput {
    pub fn new(digest: String, sid: String, ty: String) -> Self {
        Self {
            digest,
            sid,
            ty,
            comment: Default::default(),
            group: Default::default(),
            max_relocate: Default::default(),
            max_restart: Default::default(),
            state: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description."]
    #[doc = ""]
    pub comment: Option<String>,
    #[doc = "Can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The HA group identifier."]
    #[doc = ""]
    pub group: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of service relocate tries when a service failes to start."]
    #[doc = ""]
    pub max_relocate: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of tries to restart the service on a node after its start failed."]
    #[doc = ""]
    pub max_restart: Option<i64>,
    #[doc = "HA resource ID. This consists of a resource type followed by a resource specific name, separated with colon (example: vm:100 / ct:100). For virtual machines and containers, you can simply use the VM or CT id as a shortcut (example: 100)."]
    #[doc = ""]
    pub sid: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Requested resource state."]
    #[doc = ""]
    pub state: Option<State>,
    #[serde(rename = "type")]
    #[doc = "The type of the resources."]
    #[doc = ""]
    pub ty: String,
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
    #[doc = "Description."]
    #[doc = ""]
    pub comment: Option<CommentStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<DeleteStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The HA group identifier."]
    #[doc = ""]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of service relocate tries when a service failes to start."]
    #[doc = ""]
    pub max_relocate: Option<MaxRelocateInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of tries to restart the service on a node after its start failed."]
    #[doc = ""]
    pub max_restart: Option<MaxRestartInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Requested resource state."]
    #[doc = ""]
    #[doc = "Requested resource state. The CRM reads this state and acts accordingly."]
    #[doc = ""]
    #[doc = "Please note that `enabled` is just an alias for `started`."]
    #[doc = ""]
    #[doc = "`started`;;"]
    #[doc = ""]
    #[doc = "The CRM tries to start the resource. Service state is"]
    #[doc = ""]
    #[doc = "set to `started` after successful start. On node failures, or when start"]
    #[doc = ""]
    #[doc = "fails, it tries to recover the resource.  If everything fails, service"]
    #[doc = ""]
    #[doc = "state it set to `error`."]
    #[doc = ""]
    #[doc = "`stopped`;;"]
    #[doc = ""]
    #[doc = "The CRM tries to keep the resource in `stopped` state, but it"]
    #[doc = ""]
    #[doc = "still tries to relocate the resources on node failures."]
    #[doc = ""]
    #[doc = "`disabled`;;"]
    #[doc = ""]
    #[doc = "The CRM tries to put the resource in `stopped` state, but does not try"]
    #[doc = ""]
    #[doc = "to relocate the resources on node failures. The main purpose of this"]
    #[doc = ""]
    #[doc = "state is error recovery, because it is the only way to move a resource out"]
    #[doc = ""]
    #[doc = "of the `error` state."]
    #[doc = ""]
    #[doc = "`ignored`;;"]
    #[doc = ""]
    #[doc = "The resource gets removed from the manager status and so the CRM and the LRM do"]
    #[doc = ""]
    #[doc = "not touch the resource anymore. All {pve} API calls affecting this resource"]
    #[doc = ""]
    #[doc = "will be executed, directly bypassing the HA stack. CRM commands will be thrown"]
    #[doc = ""]
    #[doc = "away while there source is in this state. The resource will not get relocated"]
    #[doc = ""]
    #[doc = "on node failures."]
    #[doc = ""]
    pub state: Option<State2>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Requested resource state."]
#[doc = ""]
pub enum State {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "stopped")]
    Stopped,
}
impl TryFrom<&str> for State {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "disabled" => Ok(Self::Disabled),
            "enabled" => Ok(Self::Enabled),
            "ignored" => Ok(Self::Ignored),
            "started" => Ok(Self::Started),
            "stopped" => Ok(Self::Stopped),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Requested resource state."]
#[doc = ""]
#[doc = "Requested resource state. The CRM reads this state and acts accordingly."]
#[doc = ""]
#[doc = "Please note that `enabled` is just an alias for `started`."]
#[doc = ""]
#[doc = "`started`;;"]
#[doc = ""]
#[doc = "The CRM tries to start the resource. Service state is"]
#[doc = ""]
#[doc = "set to `started` after successful start. On node failures, or when start"]
#[doc = ""]
#[doc = "fails, it tries to recover the resource.  If everything fails, service"]
#[doc = ""]
#[doc = "state it set to `error`."]
#[doc = ""]
#[doc = "`stopped`;;"]
#[doc = ""]
#[doc = "The CRM tries to keep the resource in `stopped` state, but it"]
#[doc = ""]
#[doc = "still tries to relocate the resources on node failures."]
#[doc = ""]
#[doc = "`disabled`;;"]
#[doc = ""]
#[doc = "The CRM tries to put the resource in `stopped` state, but does not try"]
#[doc = ""]
#[doc = "to relocate the resources on node failures. The main purpose of this"]
#[doc = ""]
#[doc = "state is error recovery, because it is the only way to move a resource out"]
#[doc = ""]
#[doc = "of the `error` state."]
#[doc = ""]
#[doc = "`ignored`;;"]
#[doc = ""]
#[doc = "The resource gets removed from the manager status and so the CRM and the LRM do"]
#[doc = ""]
#[doc = "not touch the resource anymore. All {pve} API calls affecting this resource"]
#[doc = ""]
#[doc = "will be executed, directly bypassing the HA stack. CRM commands will be thrown"]
#[doc = ""]
#[doc = "away while there source is in this state. The resource will not get relocated"]
#[doc = ""]
#[doc = "on node failures."]
#[doc = ""]
pub enum State2 {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "started")]
    #[default]
    Started,
    #[serde(rename = "stopped")]
    Stopped,
}
impl TryFrom<&str> for State2 {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "disabled" => Ok(Self::Disabled),
            "enabled" => Ok(Self::Enabled),
            "ignored" => Ok(Self::Ignored),
            "started" => Ok(Self::Started),
            "stopped" => Ok(Self::Stopped),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MaxRelocateInt(i128);
impl crate::types::bounded_integer::BoundedInteger for MaxRelocateInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(1i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for MaxRelocateInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for MaxRelocateInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MaxRelocateInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MaxRestartInt(i128);
impl crate::types::bounded_integer::BoundedInteger for MaxRestartInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(1i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for MaxRestartInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for MaxRestartInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MaxRestartInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
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
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CommentStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DeleteStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DeleteStr {
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
impl std::convert::TryFrom<String> for DeleteStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DeleteStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DeleteStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DigestStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DigestStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(64usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 64";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DigestStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DigestStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DigestStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    pub fn migrate(&self) -> migrate::MigrateClient<T> {
        migrate::MigrateClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SidClient<T>
where
    T: crate::client::Client,
{
    pub fn relocate(&self) -> relocate::RelocateClient<T> {
        relocate::RelocateClient::<T>::new(self.client.clone(), &self.path)
    }
}
