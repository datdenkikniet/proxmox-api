pub struct DnsClient<T> {
    client: T,
    path: String,
}
impl<T> DnsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/dns"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "First name server IP address."]
    pub dns1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Second name server IP address."]
    pub dns2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Third name server IP address."]
    pub dns3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Search domain for host-name lookup."]
    pub search: Option<String>,
}
impl<T> DnsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read DNS settings."]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PutParams {
    pub fn new(search: String) -> Self {
        Self {
            search,
            dns1: Default::default(),
            dns2: Default::default(),
            dns3: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "First name server IP address."]
    pub dns1: Option<::std::net::IpAddr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Second name server IP address."]
    pub dns2: Option<::std::net::IpAddr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Third name server IP address."]
    pub dns3: Option<::std::net::IpAddr>,
    #[doc = "Search domain for host-name lookup."]
    pub search: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> DnsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Write DNS settings."]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
