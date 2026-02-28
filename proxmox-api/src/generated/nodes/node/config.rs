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
    #[doc = "Get node configuration options."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set node configuration options."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node specific ACME settings."]
    #[doc = ""]
    pub acme: Option<String>,
    #[serde(rename = "acmedomain[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedAcmedomains, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedAcmedomains, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "ACME domain and validation plugin"]
    #[doc = ""]
    pub acmedomains: ::std::collections::HashMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the Node. Shown in the web-interface node notes panel. This is saved as comment inside the configuration file."]
    #[doc = ""]
    pub description: Option<DescriptionStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(rename = "startall-onboot-delay")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled."]
    #[doc = ""]
    pub startall_onboot_delay: Option<StartallOnbootDelayInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node specific wake on LAN settings."]
    #[doc = ""]
    pub wakeonlan: Option<String>,
    #[serde(
        flatten,
        deserialize_with = "crate::types::multi::deserialize_additional_data::<'_, GetOutput, _, _>"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl crate::types::multi::Test for GetOutput {
    fn test_fn() -> fn(&str) -> bool {
        fn the_test(input: &str) -> bool {
            let array = [
                <NumberedAcmedomains as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
            ];
            array.iter().any(|f| f(input))
        }
        the_test as _
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Return only a specific property from the node configuration."]
    #[doc = ""]
    pub property: Option<Property>,
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
    #[doc = "Node specific ACME settings."]
    #[doc = ""]
    pub acme: Option<String>,
    #[serde(rename = "acmedomain[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedAcmedomains, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedAcmedomains, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "ACME domain and validation plugin"]
    #[doc = ""]
    pub acmedomains: ::std::collections::HashMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the Node. Shown in the web-interface node notes panel. This is saved as comment inside the configuration file."]
    #[doc = ""]
    pub description: Option<DescriptionStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(rename = "startall-onboot-delay")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled."]
    #[doc = ""]
    pub startall_onboot_delay: Option<StartallOnbootDelayInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node specific wake on LAN settings."]
    #[doc = ""]
    pub wakeonlan: Option<String>,
    #[serde(
        flatten,
        deserialize_with = "crate::types::multi::deserialize_additional_data::<'_, PutParams, _, _>"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl crate::types::multi::Test for PutParams {
    fn test_fn() -> fn(&str) -> bool {
        fn the_test(input: &str) -> bool {
            let array = [
                <NumberedAcmedomains as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
            ];
            array.iter().any(|f| f(input))
        }
        the_test as _
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Return only a specific property from the node configuration."]
#[doc = ""]
pub enum Property {
    #[serde(rename = "acme")]
    Acme,
    #[serde(rename = "acmedomain0")]
    Acmedomain0,
    #[serde(rename = "acmedomain1")]
    Acmedomain1,
    #[serde(rename = "acmedomain2")]
    Acmedomain2,
    #[serde(rename = "acmedomain3")]
    Acmedomain3,
    #[serde(rename = "acmedomain4")]
    Acmedomain4,
    #[serde(rename = "acmedomain5")]
    Acmedomain5,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "startall-onboot-delay")]
    StartallOnbootDelay,
    #[serde(rename = "wakeonlan")]
    Wakeonlan,
}
impl TryFrom<&str> for Property {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "acme" => Ok(Self::Acme),
            "acmedomain0" => Ok(Self::Acmedomain0),
            "acmedomain1" => Ok(Self::Acmedomain1),
            "acmedomain2" => Ok(Self::Acmedomain2),
            "acmedomain3" => Ok(Self::Acmedomain3),
            "acmedomain4" => Ok(Self::Acmedomain4),
            "acmedomain5" => Ok(Self::Acmedomain5),
            "description" => Ok(Self::Description),
            "startall-onboot-delay" => Ok(Self::StartallOnbootDelay),
            "wakeonlan" => Ok(Self::Wakeonlan),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Default)]
struct NumberedAcmedomains;
impl crate::types::multi::NumberedItems for NumberedAcmedomains {
    type Item = String;
    const PREFIX: &'static str = "acmedomain";
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct StartallOnbootDelayInt(i128);
impl crate::types::bounded_integer::BoundedInteger for StartallOnbootDelayInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = Some(300i128);
    const DEFAULT: Option<i128> = Some(0i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 0 and 300";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for StartallOnbootDelayInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for StartallOnbootDelayInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for StartallOnbootDelayInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DescriptionStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DescriptionStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(65536usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 65536";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DescriptionStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DescriptionStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DescriptionStr {
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
