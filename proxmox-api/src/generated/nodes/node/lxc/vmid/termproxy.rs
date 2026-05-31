#[derive(Debug, Clone)]
pub struct TermproxyClient<T> {
    client: T,
    path: String,
}
impl<T> TermproxyClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/termproxy"),
        }
    }
}
impl<T> TermproxyClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Creates a TCP proxy connection."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/vms/{vmid}\", [\"VM.Console\"])"]
    pub async fn post(&self) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &()).await
    }
}
impl PostOutput {
    pub fn new(port: i64, ticket: String, upid: String, user: String) -> Self {
        Self {
            port,
            ticket,
            upid,
            user,
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub port: i64,
    pub ticket: String,
    pub upid: String,
    pub user: String,
}
