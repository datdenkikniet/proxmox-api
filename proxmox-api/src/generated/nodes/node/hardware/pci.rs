pub mod pciid;
pub struct PciClient<T> {
    client: T,
    path: String,
}
impl<T> PciClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/pci"),
        }
    }
}
impl<T> PciClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List local PCI devices."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(class: String, device: String, id: String, iommugroup: u64, vendor: String) -> Self {
        Self {
            class,
            device,
            id,
            iommugroup,
            vendor,
            device_name: Default::default(),
            mdev: Default::default(),
            subsystem_device: Default::default(),
            subsystem_device_name: Default::default(),
            subsystem_vendor: Default::default(),
            subsystem_vendor_name: Default::default(),
            vendor_name: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The PCI Class of the device."]
    #[doc = ""]
    pub class: String,
    #[doc = "The Device ID."]
    #[doc = ""]
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub device_name: Option<String>,
    #[doc = "The PCI ID."]
    #[doc = ""]
    pub id: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The IOMMU group in which the device is in. If no IOMMU group is detected, it is set to -1."]
    #[doc = ""]
    pub iommugroup: u64,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If set, marks that the device is capable of creating mediated devices."]
    #[doc = ""]
    pub mdev: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The Subsystem Device ID."]
    #[doc = ""]
    pub subsystem_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subsystem_device_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The Subsystem Vendor ID."]
    #[doc = ""]
    pub subsystem_vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subsystem_vendor_name: Option<String>,
    #[doc = "The Vendor ID."]
    #[doc = ""]
    pub vendor: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vendor_name: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "pci-class-blacklist")]
    #[serde(
        serialize_with = "crate::types::serialize_list",
        deserialize_with = "crate::types::deserialize_list"
    )]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of blacklisted PCI classes, which will not be returned. Following are filtered by default: Memory Controller (05), Bridge (06) and Processor (0b)."]
    #[doc = ""]
    pub pci_class_blacklist: Vec<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If disabled, does only print the PCI IDs. Otherwise, additional information like vendor and device will be returned."]
    #[doc = ""]
    pub verbose: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> PciClient<T>
where
    T: crate::client::Client,
{
    pub fn pciid(&self, pciid: &str) -> pciid::PciidClient<T> {
        pciid::PciidClient::<T>::new(self.client.clone(), &self.path, pciid)
    }
}
