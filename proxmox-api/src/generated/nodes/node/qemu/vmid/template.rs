pub struct TemplateClient<T> {
    client: T,
    path: String,
}
impl<T> TemplateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/template"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a TemplateClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> TemplateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a Template."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If you want to convert only 1 disk to base image."]
    #[doc = ""]
    pub disk: Option<Disk>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "If you want to convert only 1 disk to base image."]
#[doc = ""]
pub enum Disk {
    #[serde(rename = "efidisk0")]
    Efidisk0,
    #[serde(rename = "ide0")]
    Ide0,
    #[serde(rename = "ide1")]
    Ide1,
    #[serde(rename = "ide2")]
    Ide2,
    #[serde(rename = "ide3")]
    Ide3,
    #[serde(rename = "sata0")]
    Sata0,
    #[serde(rename = "sata1")]
    Sata1,
    #[serde(rename = "sata2")]
    Sata2,
    #[serde(rename = "sata3")]
    Sata3,
    #[serde(rename = "sata4")]
    Sata4,
    #[serde(rename = "sata5")]
    Sata5,
    #[serde(rename = "scsi0")]
    Scsi0,
    #[serde(rename = "scsi1")]
    Scsi1,
    #[serde(rename = "scsi10")]
    Scsi10,
    #[serde(rename = "scsi11")]
    Scsi11,
    #[serde(rename = "scsi12")]
    Scsi12,
    #[serde(rename = "scsi13")]
    Scsi13,
    #[serde(rename = "scsi14")]
    Scsi14,
    #[serde(rename = "scsi15")]
    Scsi15,
    #[serde(rename = "scsi16")]
    Scsi16,
    #[serde(rename = "scsi17")]
    Scsi17,
    #[serde(rename = "scsi18")]
    Scsi18,
    #[serde(rename = "scsi19")]
    Scsi19,
    #[serde(rename = "scsi2")]
    Scsi2,
    #[serde(rename = "scsi20")]
    Scsi20,
    #[serde(rename = "scsi21")]
    Scsi21,
    #[serde(rename = "scsi22")]
    Scsi22,
    #[serde(rename = "scsi23")]
    Scsi23,
    #[serde(rename = "scsi24")]
    Scsi24,
    #[serde(rename = "scsi25")]
    Scsi25,
    #[serde(rename = "scsi26")]
    Scsi26,
    #[serde(rename = "scsi27")]
    Scsi27,
    #[serde(rename = "scsi28")]
    Scsi28,
    #[serde(rename = "scsi29")]
    Scsi29,
    #[serde(rename = "scsi3")]
    Scsi3,
    #[serde(rename = "scsi30")]
    Scsi30,
    #[serde(rename = "scsi4")]
    Scsi4,
    #[serde(rename = "scsi5")]
    Scsi5,
    #[serde(rename = "scsi6")]
    Scsi6,
    #[serde(rename = "scsi7")]
    Scsi7,
    #[serde(rename = "scsi8")]
    Scsi8,
    #[serde(rename = "scsi9")]
    Scsi9,
    #[serde(rename = "tpmstate0")]
    Tpmstate0,
    #[serde(rename = "virtio0")]
    Virtio0,
    #[serde(rename = "virtio1")]
    Virtio1,
    #[serde(rename = "virtio10")]
    Virtio10,
    #[serde(rename = "virtio11")]
    Virtio11,
    #[serde(rename = "virtio12")]
    Virtio12,
    #[serde(rename = "virtio13")]
    Virtio13,
    #[serde(rename = "virtio14")]
    Virtio14,
    #[serde(rename = "virtio15")]
    Virtio15,
    #[serde(rename = "virtio2")]
    Virtio2,
    #[serde(rename = "virtio3")]
    Virtio3,
    #[serde(rename = "virtio4")]
    Virtio4,
    #[serde(rename = "virtio5")]
    Virtio5,
    #[serde(rename = "virtio6")]
    Virtio6,
    #[serde(rename = "virtio7")]
    Virtio7,
    #[serde(rename = "virtio8")]
    Virtio8,
    #[serde(rename = "virtio9")]
    Virtio9,
}
impl TryFrom<&str> for Disk {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "efidisk0" => Ok(Self::Efidisk0),
            "ide0" => Ok(Self::Ide0),
            "ide1" => Ok(Self::Ide1),
            "ide2" => Ok(Self::Ide2),
            "ide3" => Ok(Self::Ide3),
            "sata0" => Ok(Self::Sata0),
            "sata1" => Ok(Self::Sata1),
            "sata2" => Ok(Self::Sata2),
            "sata3" => Ok(Self::Sata3),
            "sata4" => Ok(Self::Sata4),
            "sata5" => Ok(Self::Sata5),
            "scsi0" => Ok(Self::Scsi0),
            "scsi1" => Ok(Self::Scsi1),
            "scsi10" => Ok(Self::Scsi10),
            "scsi11" => Ok(Self::Scsi11),
            "scsi12" => Ok(Self::Scsi12),
            "scsi13" => Ok(Self::Scsi13),
            "scsi14" => Ok(Self::Scsi14),
            "scsi15" => Ok(Self::Scsi15),
            "scsi16" => Ok(Self::Scsi16),
            "scsi17" => Ok(Self::Scsi17),
            "scsi18" => Ok(Self::Scsi18),
            "scsi19" => Ok(Self::Scsi19),
            "scsi2" => Ok(Self::Scsi2),
            "scsi20" => Ok(Self::Scsi20),
            "scsi21" => Ok(Self::Scsi21),
            "scsi22" => Ok(Self::Scsi22),
            "scsi23" => Ok(Self::Scsi23),
            "scsi24" => Ok(Self::Scsi24),
            "scsi25" => Ok(Self::Scsi25),
            "scsi26" => Ok(Self::Scsi26),
            "scsi27" => Ok(Self::Scsi27),
            "scsi28" => Ok(Self::Scsi28),
            "scsi29" => Ok(Self::Scsi29),
            "scsi3" => Ok(Self::Scsi3),
            "scsi30" => Ok(Self::Scsi30),
            "scsi4" => Ok(Self::Scsi4),
            "scsi5" => Ok(Self::Scsi5),
            "scsi6" => Ok(Self::Scsi6),
            "scsi7" => Ok(Self::Scsi7),
            "scsi8" => Ok(Self::Scsi8),
            "scsi9" => Ok(Self::Scsi9),
            "tpmstate0" => Ok(Self::Tpmstate0),
            "virtio0" => Ok(Self::Virtio0),
            "virtio1" => Ok(Self::Virtio1),
            "virtio10" => Ok(Self::Virtio10),
            "virtio11" => Ok(Self::Virtio11),
            "virtio12" => Ok(Self::Virtio12),
            "virtio13" => Ok(Self::Virtio13),
            "virtio14" => Ok(Self::Virtio14),
            "virtio15" => Ok(Self::Virtio15),
            "virtio2" => Ok(Self::Virtio2),
            "virtio3" => Ok(Self::Virtio3),
            "virtio4" => Ok(Self::Virtio4),
            "virtio5" => Ok(Self::Virtio5),
            "virtio6" => Ok(Self::Virtio6),
            "virtio7" => Ok(Self::Virtio7),
            "virtio8" => Ok(Self::Virtio8),
            "virtio9" => Ok(Self::Virtio9),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
