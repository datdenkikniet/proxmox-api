pub struct ImportMetadataClient<T> {
    client: T,
    path: String,
}
impl<T> ImportMetadataClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/import-metadata"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Source {
    #[serde(rename = "esxi")]
    Esxi,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "vm")]
    Vm,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum WarningType {
    #[serde(rename = "cdrom-image-ignored")]
    CdromImageIgnored,
    #[serde(rename = "efi-state-lost")]
    EfiStateLost,
    #[serde(rename = "guest-is-running")]
    GuestIsRunning,
    #[serde(rename = "nvme-unsupported")]
    NvmeUnsupported,
    #[serde(rename = "ovmf-with-lsi-unsupported")]
    OvmfWithLsiUnsupported,
    #[serde(rename = "serial-port-socket-only")]
    SerialPortSocketOnly,
}
impl GetParams {
    pub fn new(volume: String) -> Self {
        Self {
            volume,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "Volume identifier for the guest archive/entry."]
    pub volume: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct CreateArgsGetOutputCreateArgs {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DisksGetOutputDisks {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct NetGetOutputNet {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl WarningsGetOutputWarningsItems {
    pub fn new(ty: WarningType) -> Self {
        Self {
            ty,
            key: Default::default(),
            value: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct WarningsGetOutputWarningsItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Related subject (config) key of warning."]
    pub key: Option<String>,
    #[serde(rename = "type")]
    #[doc = "What this warning is about."]
    pub ty: WarningType,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Related subject (config) value of warning."]
    pub value: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(create_args: CreateArgsGetOutputCreateArgs, source: Source, ty: Type) -> Self {
        Self {
            create_args,
            source,
            ty,
            disks: Default::default(),
            net: Default::default(),
            warnings: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(rename = "create-args")]
    #[doc = "Parameters which can be used in a call to create a VM or container."]
    pub create_args: CreateArgsGetOutputCreateArgs,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Recognised disk volumes as `$bus$id` =\\> `$storeid:$path` map."]
    pub disks: Option<DisksGetOutputDisks>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Recognised network interfaces as `net$id` =\\> { ...params } object."]
    pub net: Option<NetGetOutputNet>,
    #[doc = "The type of the import-source of this guest volume."]
    pub source: Source,
    #[serde(rename = "type")]
    #[doc = "The type of guest this is going to produce."]
    pub ty: Type,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of known issues that can affect the import of a guest. Note that lack of warning does not imply that there cannot be any problems."]
    pub warnings: Vec<WarningsGetOutputWarningsItems>,
}
impl<T> ImportMetadataClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the base parameters for creating a guest which imports data from a foreign importable guest, like an ESXi VM"]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
