pub struct RrdClient<T> {
    client: T,
    path: String,
}
impl<T> RrdClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/rrd"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a RrdClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> RrdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read VM RRD statistics (returns PNG)"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<GetParams, GetOutput, T::Error> for &RrdClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        self.get(params)
    }
}
impl GetOutput {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    pub filename: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(ds: String, timeframe: Timeframe) -> Self {
        Self {
            ds,
            timeframe,
            cf: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The RRD consolidation function"]
    #[doc = ""]
    pub cf: Option<Cf>,
    #[doc = "The list of datasources you want to display."]
    #[doc = ""]
    pub ds: String,
    #[doc = "Specify the time frame you are interested in."]
    #[doc = ""]
    pub timeframe: Timeframe,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "The RRD consolidation function"]
#[doc = ""]
pub enum Cf {
    AVERAGE,
    MAX,
}
impl TryFrom<&str> for Cf {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "AVERAGE" => Ok(Self::AVERAGE),
            "MAX" => Ok(Self::MAX),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Specify the time frame you are interested in."]
#[doc = ""]
pub enum Timeframe {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "year")]
    Year,
}
impl TryFrom<&str> for Timeframe {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),
            "year" => Ok(Self::Year),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
