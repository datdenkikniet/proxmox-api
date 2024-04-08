pub struct ExecStatusClient<T> {
    client: T,
    path: String,
}
impl<T> ExecStatusClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/exec-status"),
        }
    }
}
impl<T> ExecStatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Gets the status of the given pid started by the guest-agent"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutput {
    pub fn new(exited: bool) -> Self {
        Self {
            exited,
            err_data: Default::default(),
            err_truncated: Default::default(),
            exitcode: Default::default(),
            out_data: Default::default(),
            out_truncated: Default::default(),
            signal: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(rename = "err-data")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "stderr of the process"]
    #[doc = ""]
    pub err_data: Option<String>,
    #[serde(rename = "err-truncated")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "true if stderr was not fully captured"]
    #[doc = ""]
    pub err_truncated: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "process exit code if it was normally terminated."]
    #[doc = ""]
    pub exitcode: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "Tells if the given command has exited yet."]
    #[doc = ""]
    pub exited: bool,
    #[serde(rename = "out-data")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "stdout of the process"]
    #[doc = ""]
    pub out_data: Option<String>,
    #[serde(rename = "out-truncated")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "true if stdout was not fully captured"]
    #[doc = ""]
    pub out_truncated: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "signal number or exception code if the process was abnormally terminated."]
    #[doc = ""]
    pub signal: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(pid: u64) -> Self {
        Self {
            pid,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The PID to query"]
    #[doc = ""]
    pub pid: u64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
