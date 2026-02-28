pub mod upid;
#[derive(Debug, Clone)]
pub struct TasksClient<T> {
    client: T,
    path: String,
}
impl<T> TasksClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/tasks"),
        }
    }
}
impl<T> TasksClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read task list for one node (finished tasks)."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(
        id: String,
        node: String,
        pid: i64,
        pstart: i64,
        starttime: i64,
        ty: String,
        upid: String,
        user: String,
    ) -> Self {
        Self {
            id,
            node,
            pid,
            pstart,
            starttime,
            ty,
            upid,
            user,
            endtime: Default::default(),
            status: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub endtime: Option<i64>,
    pub id: String,
    pub node: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub pid: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub pstart: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub starttime: i64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
    #[serde(rename = "type")]
    pub ty: String,
    pub upid: String,
    pub user: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks with a status of ERROR."]
    #[doc = ""]
    pub errors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list this amount of tasks."]
    #[doc = ""]
    pub limit: Option<LimitInt>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks since this UNIX epoch."]
    #[doc = ""]
    pub since: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List archived, active or all tasks."]
    #[doc = ""]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List tasks beginning from this offset."]
    #[doc = ""]
    pub start: Option<StartInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of Task States that should be returned."]
    #[doc = ""]
    pub statusfilter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks of this type (e.g., vzstart, vzdump)."]
    #[doc = ""]
    pub typefilter: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks until this UNIX epoch."]
    #[doc = ""]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks from this user."]
    #[doc = ""]
    pub userfilter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks for this VM."]
    #[doc = ""]
    pub vmid: Option<VmidInt>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "List archived, active or all tasks."]
#[doc = ""]
pub enum Source {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "archive")]
    #[default]
    Archive,
}
impl TryFrom<&str> for Source {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "active" => Ok(Self::Active),
            "all" => Ok(Self::All),
            "archive" => Ok(Self::Archive),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LimitInt(i128);
impl crate::types::bounded_integer::BoundedInteger for LimitInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(50i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for LimitInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for LimitInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for LimitInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct StartInt(i128);
impl crate::types::bounded_integer::BoundedInteger for StartInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(0i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for StartInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for StartInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for StartInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VmidInt(i128);
impl crate::types::bounded_integer::BoundedInteger for VmidInt {
    const MIN: Option<i128> = Some(100i128);
    const MAX: Option<i128> = Some(999999999i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 100 and 999999999";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for VmidInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for VmidInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VmidInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
impl<T> TasksClient<T>
where
    T: crate::client::Client,
{
    pub fn upid(&self, upid: &str) -> upid::UpidClient<T> {
        upid::UpidClient::<T>::new(self.client.clone(), &self.path, upid)
    }
}
