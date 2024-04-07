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
impl<T> TemplateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a Template."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If you want to convert only 1 disk to base image."]
    pub disk: Option<Disk>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
