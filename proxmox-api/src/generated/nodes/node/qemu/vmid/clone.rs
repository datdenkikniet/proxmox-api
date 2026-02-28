#[derive(Debug, Clone)]
pub struct CloneClient<T> {
    client: T,
    path: String,
}
impl<T> CloneClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/clone"),
        }
    }
}
impl<T> CloneClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a copy of virtual machine/template."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(newid: NewidInt) -> Self {
        Self {
            newid,
            bwlimit: Default::default(),
            description: Default::default(),
            format: Default::default(),
            full: Default::default(),
            name: Default::default(),
            pool: Default::default(),
            snapname: Default::default(),
            storage: Default::default(),
            target: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<BwlimitInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the new VM."]
    #[doc = ""]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target format for file storage. Only valid for full clone."]
    #[doc = ""]
    pub format: Option<Format>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create a full copy of all disks. This is always done when you clone a normal VM. For VM templates, we try to create a linked clone by default."]
    #[doc = ""]
    pub full: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set a name for the new VM."]
    #[doc = ""]
    pub name: Option<String>,
    #[doc = "VMID for the clone."]
    #[doc = ""]
    pub newid: NewidInt,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Add the new VM to the specified pool."]
    #[doc = ""]
    pub pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The name of the snapshot."]
    #[doc = ""]
    pub snapname: Option<SnapnameStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target storage for full clone."]
    #[doc = ""]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target node. Only allowed if the original VM is on shared storage."]
    #[doc = ""]
    pub target: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Target format for file storage. Only valid for full clone."]
#[doc = ""]
pub enum Format {
    #[serde(rename = "qcow2")]
    Qcow2,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "vmdk")]
    Vmdk,
}
impl TryFrom<&str> for Format {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "qcow2" => Ok(Self::Qcow2),
            "raw" => Ok(Self::Raw),
            "vmdk" => Ok(Self::Vmdk),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BwlimitInt(i128);
impl crate::types::bounded_integer::BoundedInteger for BwlimitInt {
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
impl std::convert::TryFrom<i128> for BwlimitInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for BwlimitInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for BwlimitInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NewidInt(i128);
impl crate::types::bounded_integer::BoundedInteger for NewidInt {
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
impl std::convert::TryFrom<i128> for NewidInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for NewidInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for NewidInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
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
