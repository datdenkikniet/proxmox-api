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
impl<'a, T> crate::ProxmoxClient for &'a ConfigClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get node configuration options."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
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
        let path = crate::ProxmoxClient::path(self).as_ref();
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
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(rename = "startall-onboot-delay")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled."]
    #[doc = ""]
    pub startall_onboot_delay: Option<u64>,
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
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(rename = "startall-onboot-delay")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled."]
    #[doc = ""]
    pub startall_onboot_delay: Option<u64>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
