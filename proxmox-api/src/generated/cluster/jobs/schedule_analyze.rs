#[derive(Debug, Clone)]
pub struct ScheduleAnalyzeClient<T> {
    client: T,
    path: String,
}
impl<T> ScheduleAnalyzeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/schedule-analyze"),
        }
    }
}
impl<T> ScheduleAnalyzeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns a list of future schedule runtimes."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(timestamp: i64, utc: String) -> Self {
        Self {
            timestamp,
            utc,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "UNIX timestamp for the run."]
    #[doc = ""]
    pub timestamp: i64,
    #[doc = "UTC timestamp for the run."]
    #[doc = ""]
    pub utc: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(schedule: String) -> Self {
        Self {
            schedule,
            iterations: Default::default(),
            starttime: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of event-iteration to simulate and return."]
    #[doc = ""]
    pub iterations: Option<i64>,
    #[doc = "Job schedule. The format is a subset of `systemd` calendar events."]
    #[doc = ""]
    pub schedule: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "UNIX timestamp to start the calculation from. Defaults to the current time."]
    #[doc = ""]
    pub starttime: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
