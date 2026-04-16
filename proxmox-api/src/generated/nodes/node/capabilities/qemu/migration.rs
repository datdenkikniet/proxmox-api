#[derive(Debug, Clone)]
pub struct MigrationClient<T> {
    client: T,
    path: String,
}
impl<T> MigrationClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/migration"),
        }
    }
}
impl<T> MigrationClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get node-specific QEMU migration capabilities of the node. Requires the 'Sys.Audit' permission on '/nodes/\\<node\\>'."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutput {
    pub fn new(has_dbus_vmstate: bool) -> Self {
        Self { has_dbus_vmstate }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(rename = "has-dbus-vmstate")]
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "Whether the host supports live-migrating additional VM state via the dbus-vmstate helper."]
    #[doc = ""]
    pub has_dbus_vmstate: bool,
}
