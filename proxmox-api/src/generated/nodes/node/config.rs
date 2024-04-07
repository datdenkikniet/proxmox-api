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
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node specific ACME settings."]
    pub acme: Option<String>,
    #[serde(rename = "acmedomain[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedAcmedomains, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedAcmedomains, _>"
    )]
    #[serde(
        skip_serializing_if = "::std::collections::BTreeMap::is_empty",
        default
    )]
    #[serde(flatten)]
    #[doc = "ACME domain and validation plugin"]
    pub acmedomains: ::std::collections::BTreeMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the Node. Shown in the web-interface node notes panel. This is saved as comment inside the configuration file."]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    pub digest: Option<String>,
    #[serde(rename = "startall-onboot-delay")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled."]
    pub startall_onboot_delay: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node specific wake on LAN settings."]
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
    pub acme: Option<String>,
    #[serde(rename = "acmedomain[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedAcmedomains, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedAcmedomains, _>"
    )]
    #[serde(
        skip_serializing_if = "::std::collections::BTreeMap::is_empty",
        default
    )]
    #[serde(flatten)]
    #[doc = "ACME domain and validation plugin"]
    pub acmedomains: ::std::collections::BTreeMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the Node. Shown in the web-interface node notes panel. This is saved as comment inside the configuration file."]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    pub digest: Option<String>,
    #[serde(rename = "startall-onboot-delay")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Initial delay in seconds, before starting all the Virtual Guests with on-boot enabled."]
    pub startall_onboot_delay: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node specific wake on LAN settings."]
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
#[derive(Default)]
struct NumberedAcmedomains;
impl crate::types::multi::NumberedItems for NumberedAcmedomains {
    type Item = String;
    const PREFIX: &'static str = "acmedomain";
}
