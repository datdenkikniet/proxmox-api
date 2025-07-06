#[derive(Debug, Clone)]
pub struct ExportClient<T> {
    client: T,
    path: String,
}
impl<T> ExportClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/export"),
        }
    }
}
impl<T> ExportClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Retrieve metrics of the cluster."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl DataGetOutputDataItems {
    pub fn new(id: String, metric: String, timestamp: i64, ty: Type, value: f64) -> Self {
        Self {
            id,
            metric,
            timestamp,
            ty,
            value,
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct DataGetOutputDataItems {
    #[doc = "Unique identifier for this metric object, for instance 'node/\\\\<nodename\\\\>' or 'qemu/\\\\<vmid\\\\>'."]
    #[doc = ""]
    pub id: String,
    #[doc = "Name of the metric."]
    #[doc = ""]
    pub metric: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Time at which this metric was observed"]
    #[doc = ""]
    pub timestamp: i64,
    #[serde(rename = "type")]
    #[doc = "Type of the metric."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        serialize_with = "crate::types::serialize_number",
        deserialize_with = "crate::types::deserialize_number"
    )]
    #[doc = "Metric value."]
    #[doc = ""]
    pub value: f64,
}
impl GetOutput {
    pub fn new(data: Vec<DataGetOutputDataItems>) -> Self {
        Self { data }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Array of system metrics. Metrics are sorted by their timestamp."]
    #[doc = ""]
    pub data: Vec<DataGetOutputDataItems>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Also return historic values. Returns full available metric history unless `start-time` is also set"]
    #[doc = ""]
    pub history: Option<bool>,
    #[serde(rename = "local-only")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only return metrics for the current node instead of the whole cluster"]
    #[doc = ""]
    pub local_only: Option<bool>,
    #[serde(rename = "start-time")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only include metrics with a timestamp \\\\> start-time."]
    #[doc = ""]
    pub start_time: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Type of the metric."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "counter")]
    Counter,
    #[serde(rename = "derive")]
    Derive,
    #[serde(rename = "gauge")]
    Gauge,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "counter" => Ok(Self::Counter),
            "derive" => Ok(Self::Derive),
            "gauge" => Ok(Self::Gauge),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
