#[derive(Debug, Clone)]
pub struct UsbClient<T> {
    client: T,
    path: String,
}
impl<T> UsbClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/usb"),
        }
    }
}
impl<T> UsbClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List local USB devices."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(
        busnum: i64,
        class: i64,
        devnum: i64,
        level: i64,
        port: i64,
        prodid: String,
        speed: String,
        vendid: String,
    ) -> Self {
        Self {
            busnum,
            class,
            devnum,
            level,
            port,
            prodid,
            speed,
            vendid,
            manufacturer: Default::default(),
            product: Default::default(),
            serial: Default::default(),
            usbpath: Default::default(),
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
    pub busnum: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub class: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub devnum: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub level: i64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub manufacturer: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub port: i64,
    pub prodid: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub serial: Option<String>,
    pub speed: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub usbpath: Option<String>,
    pub vendid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
