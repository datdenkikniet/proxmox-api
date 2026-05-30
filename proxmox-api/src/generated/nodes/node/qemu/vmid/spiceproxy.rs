#[derive(Debug, Clone)]
pub struct SpiceproxyClient<T> {
    client: T,
    path: String,
}
impl<T> SpiceproxyClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/spiceproxy"),
        }
    }
}
impl<T> SpiceproxyClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns a SPICE configuration to connect to the VM."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/vms/{vmid}\", [\"VM.Console\"])"]
    pub async fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params).await
    }
}
impl PostOutput {
    pub fn new(host: String, password: String, proxy: String, tls_port: i64, ty: String) -> Self {
        Self {
            host,
            password,
            proxy,
            tls_port,
            ty,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    pub host: String,
    pub password: String,
    pub proxy: String,
    #[serde(rename = "tls-port")]
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub tls_port: i64,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "SPICE proxy server. This can be used by the client to specify the proxy server. All nodes in a cluster runs 'spiceproxy', so it is up to the client to choose one. By default, we return the node where the VM is currently running. As reasonable setting is to use same node you use to connect to the API (This is window.location.hostname for the JS GUI)."]
    #[doc = ""]
    pub proxy: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
