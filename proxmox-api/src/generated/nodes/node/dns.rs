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
impl<'a, T> crate::ProxmoxClient for &'a DnsClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> DnsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read DNS settings."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> DnsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Write DNS settings."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "First name server IP address."]
    #[doc = ""]
    pub dns1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Second name server IP address."]
    #[doc = ""]
    pub dns2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Third name server IP address."]
    #[doc = ""]
    pub dns3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Search domain for host-name lookup."]
    #[doc = ""]
    pub search: Option<String>,
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
    #[doc = ""]
    pub dns1: Option<::std::net::IpAddr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Second name server IP address."]
    #[doc = ""]
    pub dns2: Option<::std::net::IpAddr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Third name server IP address."]
    #[doc = ""]
    pub dns3: Option<::std::net::IpAddr>,
    #[doc = "Search domain for host-name lookup."]
    #[doc = ""]
    pub search: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
