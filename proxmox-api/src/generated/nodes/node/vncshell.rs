#[derive(Debug, Clone)]
pub struct VncshellClient<T> {
    client: T,
    path: String,
}
impl<T> VncshellClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/vncshell"),
        }
    }
}
impl<T> VncshellClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Creates a VNC Shell proxy."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Run specific command or default to login (requires 'root@pam')"]
    #[doc = ""]
    pub cmd: Option<Cmd>,
    #[serde(rename = "cmd-opts")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Add parameters to a command. Encoded as null terminated strings."]
    #[doc = ""]
    pub cmd_opts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "sets the height of the console in pixels."]
    #[doc = ""]
    pub height: Option<HeightInt>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "use websocket instead of standard vnc."]
    #[doc = ""]
    pub websocket: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "sets the width of the console in pixels."]
    #[doc = ""]
    pub width: Option<WidthInt>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Run specific command or default to login (requires 'root@pam')"]
#[doc = ""]
pub enum Cmd {
    #[serde(rename = "ceph_install")]
    CephInstall,
    #[serde(rename = "login")]
    #[default]
    Login,
    #[serde(rename = "upgrade")]
    Upgrade,
}
impl TryFrom<&str> for Cmd {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ceph_install" => Ok(Self::CephInstall),
            "login" => Ok(Self::Login),
            "upgrade" => Ok(Self::Upgrade),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HeightInt(i128);
impl crate::types::bounded_integer::BoundedInteger for HeightInt {
    const MIN: Option<i128> = Some(16i128);
    const MAX: Option<i128> = Some(2160i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 16 and 2160";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for HeightInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for HeightInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for HeightInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WidthInt(i128);
impl crate::types::bounded_integer::BoundedInteger for WidthInt {
    const MIN: Option<i128> = Some(16i128);
    const MAX: Option<i128> = Some(4096i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 16 and 4096";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for WidthInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for WidthInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for WidthInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
