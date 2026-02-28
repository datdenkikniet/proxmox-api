pub mod id;
#[derive(Debug, Clone)]
pub struct RealmSyncClient<T> {
    client: T,
    path: String,
}
impl<T> RealmSyncClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/realm-sync"),
        }
    }
}
impl<T> RealmSyncClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List configured realm-sync-jobs."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(enabled: bool, id: String, realm: RealmStr, schedule: String) -> Self {
        Self {
            enabled,
            id,
            realm,
            schedule,
            comment: Default::default(),
            last_run: Default::default(),
            next_run: Default::default(),
            remove_vanished: Default::default(),
            scope: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A comment for the job."]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "If the job is enabled or not."]
    #[doc = ""]
    pub enabled: bool,
    #[doc = "The ID of the entry."]
    #[doc = ""]
    pub id: String,
    #[serde(rename = "last-run")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Last execution time of the job in seconds since the beginning of the UNIX epoch"]
    #[doc = ""]
    pub last_run: Option<i64>,
    #[serde(rename = "next-run")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Next planned execution time of the job in seconds since the beginning of the UNIX epoch."]
    #[doc = ""]
    pub next_run: Option<i64>,
    #[doc = "Authentication domain ID"]
    #[doc = ""]
    pub realm: RealmStr,
    #[serde(rename = "remove-vanished")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default)."]
    #[doc = ""]
    pub remove_vanished: Option<RemoveVanishedStr>,
    #[doc = "The configured sync schedule."]
    #[doc = ""]
    pub schedule: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Select what to sync."]
    #[doc = ""]
    pub scope: Option<Scope>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Select what to sync."]
#[doc = ""]
pub enum Scope {
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "groups")]
    Groups,
    #[serde(rename = "users")]
    Users,
}
impl TryFrom<&str> for Scope {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "both" => Ok(Self::Both),
            "groups" => Ok(Self::Groups),
            "users" => Ok(Self::Users),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RealmStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for RealmStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(32usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 32";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for RealmStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for RealmStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for RealmStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RemoveVanishedStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for RemoveVanishedStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = Some("none");
    const PATTERN: Option<&'static str> =
        Some("(?:(?:(?:acl|properties|entry);)*(?:acl|properties|entry))|none");
    const TYPE_DESCRIPTION: &'static str = "a string with pattern r\"(?:(?:(?:acl|properties|entry);)*(?:acl|properties|entry))|none\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for RemoveVanishedStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for RemoveVanishedStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for RemoveVanishedStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
impl<T> RealmSyncClient<T>
where
    T: crate::client::Client,
{
    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
