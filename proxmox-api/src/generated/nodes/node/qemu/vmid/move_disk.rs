pub struct MoveDiskClient<T> {
    client: T,
    path: String,
}
impl<T> MoveDiskClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/move_disk"),
        }
    }
}
impl<T> MoveDiskClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Move volume to different storage or to a different VM."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(disk: Disk) -> Self {
        Self {
            disk,
            bwlimit: Default::default(),
            delete: Default::default(),
            digest: Default::default(),
            format: Default::default(),
            storage: Default::default(),
            target_digest: Default::default(),
            target_disk: Default::default(),
            target_vmid: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<()>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Delete the original disk after successful copy. By default the original disk is kept as unused disk."]
    #[doc = ""]
    pub delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has different SHA1"]
    #[doc = ""]
    #[doc = ".\" digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[doc = "The disk you want to move."]
    #[doc = ""]
    pub disk: Disk,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target Format."]
    #[doc = ""]
    pub format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target storage."]
    #[doc = ""]
    pub storage: Option<String>,
    #[serde(rename = "target-digest")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if the current config file of the target VM has a"]
    #[doc = ""]
    #[doc = ".\" different SHA1 digest. This can be used to detect concurrent modifications."]
    #[doc = ""]
    pub target_digest: Option<String>,
    #[serde(rename = "target-disk")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The config key the disk will be moved to on the target VM (for example, ide0 or scsi1). Default is the source disk key."]
    #[doc = ""]
    pub target_disk: Option<TargetDisk>,
    #[serde(rename = "target-vmid")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The (unique) ID of the VM."]
    #[doc = ""]
    pub target_vmid: Option<crate::types::VmId>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "The disk you want to move."]
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
    #[serde(rename = "unused0")]
    Unused0,
    #[serde(rename = "unused1")]
    Unused1,
    #[serde(rename = "unused10")]
    Unused10,
    #[serde(rename = "unused100")]
    Unused100,
    #[serde(rename = "unused101")]
    Unused101,
    #[serde(rename = "unused102")]
    Unused102,
    #[serde(rename = "unused103")]
    Unused103,
    #[serde(rename = "unused104")]
    Unused104,
    #[serde(rename = "unused105")]
    Unused105,
    #[serde(rename = "unused106")]
    Unused106,
    #[serde(rename = "unused107")]
    Unused107,
    #[serde(rename = "unused108")]
    Unused108,
    #[serde(rename = "unused109")]
    Unused109,
    #[serde(rename = "unused11")]
    Unused11,
    #[serde(rename = "unused110")]
    Unused110,
    #[serde(rename = "unused111")]
    Unused111,
    #[serde(rename = "unused112")]
    Unused112,
    #[serde(rename = "unused113")]
    Unused113,
    #[serde(rename = "unused114")]
    Unused114,
    #[serde(rename = "unused115")]
    Unused115,
    #[serde(rename = "unused116")]
    Unused116,
    #[serde(rename = "unused117")]
    Unused117,
    #[serde(rename = "unused118")]
    Unused118,
    #[serde(rename = "unused119")]
    Unused119,
    #[serde(rename = "unused12")]
    Unused12,
    #[serde(rename = "unused120")]
    Unused120,
    #[serde(rename = "unused121")]
    Unused121,
    #[serde(rename = "unused122")]
    Unused122,
    #[serde(rename = "unused123")]
    Unused123,
    #[serde(rename = "unused124")]
    Unused124,
    #[serde(rename = "unused125")]
    Unused125,
    #[serde(rename = "unused126")]
    Unused126,
    #[serde(rename = "unused127")]
    Unused127,
    #[serde(rename = "unused128")]
    Unused128,
    #[serde(rename = "unused129")]
    Unused129,
    #[serde(rename = "unused13")]
    Unused13,
    #[serde(rename = "unused130")]
    Unused130,
    #[serde(rename = "unused131")]
    Unused131,
    #[serde(rename = "unused132")]
    Unused132,
    #[serde(rename = "unused133")]
    Unused133,
    #[serde(rename = "unused134")]
    Unused134,
    #[serde(rename = "unused135")]
    Unused135,
    #[serde(rename = "unused136")]
    Unused136,
    #[serde(rename = "unused137")]
    Unused137,
    #[serde(rename = "unused138")]
    Unused138,
    #[serde(rename = "unused139")]
    Unused139,
    #[serde(rename = "unused14")]
    Unused14,
    #[serde(rename = "unused140")]
    Unused140,
    #[serde(rename = "unused141")]
    Unused141,
    #[serde(rename = "unused142")]
    Unused142,
    #[serde(rename = "unused143")]
    Unused143,
    #[serde(rename = "unused144")]
    Unused144,
    #[serde(rename = "unused145")]
    Unused145,
    #[serde(rename = "unused146")]
    Unused146,
    #[serde(rename = "unused147")]
    Unused147,
    #[serde(rename = "unused148")]
    Unused148,
    #[serde(rename = "unused149")]
    Unused149,
    #[serde(rename = "unused15")]
    Unused15,
    #[serde(rename = "unused150")]
    Unused150,
    #[serde(rename = "unused151")]
    Unused151,
    #[serde(rename = "unused152")]
    Unused152,
    #[serde(rename = "unused153")]
    Unused153,
    #[serde(rename = "unused154")]
    Unused154,
    #[serde(rename = "unused155")]
    Unused155,
    #[serde(rename = "unused156")]
    Unused156,
    #[serde(rename = "unused157")]
    Unused157,
    #[serde(rename = "unused158")]
    Unused158,
    #[serde(rename = "unused159")]
    Unused159,
    #[serde(rename = "unused16")]
    Unused16,
    #[serde(rename = "unused160")]
    Unused160,
    #[serde(rename = "unused161")]
    Unused161,
    #[serde(rename = "unused162")]
    Unused162,
    #[serde(rename = "unused163")]
    Unused163,
    #[serde(rename = "unused164")]
    Unused164,
    #[serde(rename = "unused165")]
    Unused165,
    #[serde(rename = "unused166")]
    Unused166,
    #[serde(rename = "unused167")]
    Unused167,
    #[serde(rename = "unused168")]
    Unused168,
    #[serde(rename = "unused169")]
    Unused169,
    #[serde(rename = "unused17")]
    Unused17,
    #[serde(rename = "unused170")]
    Unused170,
    #[serde(rename = "unused171")]
    Unused171,
    #[serde(rename = "unused172")]
    Unused172,
    #[serde(rename = "unused173")]
    Unused173,
    #[serde(rename = "unused174")]
    Unused174,
    #[serde(rename = "unused175")]
    Unused175,
    #[serde(rename = "unused176")]
    Unused176,
    #[serde(rename = "unused177")]
    Unused177,
    #[serde(rename = "unused178")]
    Unused178,
    #[serde(rename = "unused179")]
    Unused179,
    #[serde(rename = "unused18")]
    Unused18,
    #[serde(rename = "unused180")]
    Unused180,
    #[serde(rename = "unused181")]
    Unused181,
    #[serde(rename = "unused182")]
    Unused182,
    #[serde(rename = "unused183")]
    Unused183,
    #[serde(rename = "unused184")]
    Unused184,
    #[serde(rename = "unused185")]
    Unused185,
    #[serde(rename = "unused186")]
    Unused186,
    #[serde(rename = "unused187")]
    Unused187,
    #[serde(rename = "unused188")]
    Unused188,
    #[serde(rename = "unused189")]
    Unused189,
    #[serde(rename = "unused19")]
    Unused19,
    #[serde(rename = "unused190")]
    Unused190,
    #[serde(rename = "unused191")]
    Unused191,
    #[serde(rename = "unused192")]
    Unused192,
    #[serde(rename = "unused193")]
    Unused193,
    #[serde(rename = "unused194")]
    Unused194,
    #[serde(rename = "unused195")]
    Unused195,
    #[serde(rename = "unused196")]
    Unused196,
    #[serde(rename = "unused197")]
    Unused197,
    #[serde(rename = "unused198")]
    Unused198,
    #[serde(rename = "unused199")]
    Unused199,
    #[serde(rename = "unused2")]
    Unused2,
    #[serde(rename = "unused20")]
    Unused20,
    #[serde(rename = "unused200")]
    Unused200,
    #[serde(rename = "unused201")]
    Unused201,
    #[serde(rename = "unused202")]
    Unused202,
    #[serde(rename = "unused203")]
    Unused203,
    #[serde(rename = "unused204")]
    Unused204,
    #[serde(rename = "unused205")]
    Unused205,
    #[serde(rename = "unused206")]
    Unused206,
    #[serde(rename = "unused207")]
    Unused207,
    #[serde(rename = "unused208")]
    Unused208,
    #[serde(rename = "unused209")]
    Unused209,
    #[serde(rename = "unused21")]
    Unused21,
    #[serde(rename = "unused210")]
    Unused210,
    #[serde(rename = "unused211")]
    Unused211,
    #[serde(rename = "unused212")]
    Unused212,
    #[serde(rename = "unused213")]
    Unused213,
    #[serde(rename = "unused214")]
    Unused214,
    #[serde(rename = "unused215")]
    Unused215,
    #[serde(rename = "unused216")]
    Unused216,
    #[serde(rename = "unused217")]
    Unused217,
    #[serde(rename = "unused218")]
    Unused218,
    #[serde(rename = "unused219")]
    Unused219,
    #[serde(rename = "unused22")]
    Unused22,
    #[serde(rename = "unused220")]
    Unused220,
    #[serde(rename = "unused221")]
    Unused221,
    #[serde(rename = "unused222")]
    Unused222,
    #[serde(rename = "unused223")]
    Unused223,
    #[serde(rename = "unused224")]
    Unused224,
    #[serde(rename = "unused225")]
    Unused225,
    #[serde(rename = "unused226")]
    Unused226,
    #[serde(rename = "unused227")]
    Unused227,
    #[serde(rename = "unused228")]
    Unused228,
    #[serde(rename = "unused229")]
    Unused229,
    #[serde(rename = "unused23")]
    Unused23,
    #[serde(rename = "unused230")]
    Unused230,
    #[serde(rename = "unused231")]
    Unused231,
    #[serde(rename = "unused232")]
    Unused232,
    #[serde(rename = "unused233")]
    Unused233,
    #[serde(rename = "unused234")]
    Unused234,
    #[serde(rename = "unused235")]
    Unused235,
    #[serde(rename = "unused236")]
    Unused236,
    #[serde(rename = "unused237")]
    Unused237,
    #[serde(rename = "unused238")]
    Unused238,
    #[serde(rename = "unused239")]
    Unused239,
    #[serde(rename = "unused24")]
    Unused24,
    #[serde(rename = "unused240")]
    Unused240,
    #[serde(rename = "unused241")]
    Unused241,
    #[serde(rename = "unused242")]
    Unused242,
    #[serde(rename = "unused243")]
    Unused243,
    #[serde(rename = "unused244")]
    Unused244,
    #[serde(rename = "unused245")]
    Unused245,
    #[serde(rename = "unused246")]
    Unused246,
    #[serde(rename = "unused247")]
    Unused247,
    #[serde(rename = "unused248")]
    Unused248,
    #[serde(rename = "unused249")]
    Unused249,
    #[serde(rename = "unused25")]
    Unused25,
    #[serde(rename = "unused250")]
    Unused250,
    #[serde(rename = "unused251")]
    Unused251,
    #[serde(rename = "unused252")]
    Unused252,
    #[serde(rename = "unused253")]
    Unused253,
    #[serde(rename = "unused254")]
    Unused254,
    #[serde(rename = "unused255")]
    Unused255,
    #[serde(rename = "unused26")]
    Unused26,
    #[serde(rename = "unused27")]
    Unused27,
    #[serde(rename = "unused28")]
    Unused28,
    #[serde(rename = "unused29")]
    Unused29,
    #[serde(rename = "unused3")]
    Unused3,
    #[serde(rename = "unused30")]
    Unused30,
    #[serde(rename = "unused31")]
    Unused31,
    #[serde(rename = "unused32")]
    Unused32,
    #[serde(rename = "unused33")]
    Unused33,
    #[serde(rename = "unused34")]
    Unused34,
    #[serde(rename = "unused35")]
    Unused35,
    #[serde(rename = "unused36")]
    Unused36,
    #[serde(rename = "unused37")]
    Unused37,
    #[serde(rename = "unused38")]
    Unused38,
    #[serde(rename = "unused39")]
    Unused39,
    #[serde(rename = "unused4")]
    Unused4,
    #[serde(rename = "unused40")]
    Unused40,
    #[serde(rename = "unused41")]
    Unused41,
    #[serde(rename = "unused42")]
    Unused42,
    #[serde(rename = "unused43")]
    Unused43,
    #[serde(rename = "unused44")]
    Unused44,
    #[serde(rename = "unused45")]
    Unused45,
    #[serde(rename = "unused46")]
    Unused46,
    #[serde(rename = "unused47")]
    Unused47,
    #[serde(rename = "unused48")]
    Unused48,
    #[serde(rename = "unused49")]
    Unused49,
    #[serde(rename = "unused5")]
    Unused5,
    #[serde(rename = "unused50")]
    Unused50,
    #[serde(rename = "unused51")]
    Unused51,
    #[serde(rename = "unused52")]
    Unused52,
    #[serde(rename = "unused53")]
    Unused53,
    #[serde(rename = "unused54")]
    Unused54,
    #[serde(rename = "unused55")]
    Unused55,
    #[serde(rename = "unused56")]
    Unused56,
    #[serde(rename = "unused57")]
    Unused57,
    #[serde(rename = "unused58")]
    Unused58,
    #[serde(rename = "unused59")]
    Unused59,
    #[serde(rename = "unused6")]
    Unused6,
    #[serde(rename = "unused60")]
    Unused60,
    #[serde(rename = "unused61")]
    Unused61,
    #[serde(rename = "unused62")]
    Unused62,
    #[serde(rename = "unused63")]
    Unused63,
    #[serde(rename = "unused64")]
    Unused64,
    #[serde(rename = "unused65")]
    Unused65,
    #[serde(rename = "unused66")]
    Unused66,
    #[serde(rename = "unused67")]
    Unused67,
    #[serde(rename = "unused68")]
    Unused68,
    #[serde(rename = "unused69")]
    Unused69,
    #[serde(rename = "unused7")]
    Unused7,
    #[serde(rename = "unused70")]
    Unused70,
    #[serde(rename = "unused71")]
    Unused71,
    #[serde(rename = "unused72")]
    Unused72,
    #[serde(rename = "unused73")]
    Unused73,
    #[serde(rename = "unused74")]
    Unused74,
    #[serde(rename = "unused75")]
    Unused75,
    #[serde(rename = "unused76")]
    Unused76,
    #[serde(rename = "unused77")]
    Unused77,
    #[serde(rename = "unused78")]
    Unused78,
    #[serde(rename = "unused79")]
    Unused79,
    #[serde(rename = "unused8")]
    Unused8,
    #[serde(rename = "unused80")]
    Unused80,
    #[serde(rename = "unused81")]
    Unused81,
    #[serde(rename = "unused82")]
    Unused82,
    #[serde(rename = "unused83")]
    Unused83,
    #[serde(rename = "unused84")]
    Unused84,
    #[serde(rename = "unused85")]
    Unused85,
    #[serde(rename = "unused86")]
    Unused86,
    #[serde(rename = "unused87")]
    Unused87,
    #[serde(rename = "unused88")]
    Unused88,
    #[serde(rename = "unused89")]
    Unused89,
    #[serde(rename = "unused9")]
    Unused9,
    #[serde(rename = "unused90")]
    Unused90,
    #[serde(rename = "unused91")]
    Unused91,
    #[serde(rename = "unused92")]
    Unused92,
    #[serde(rename = "unused93")]
    Unused93,
    #[serde(rename = "unused94")]
    Unused94,
    #[serde(rename = "unused95")]
    Unused95,
    #[serde(rename = "unused96")]
    Unused96,
    #[serde(rename = "unused97")]
    Unused97,
    #[serde(rename = "unused98")]
    Unused98,
    #[serde(rename = "unused99")]
    Unused99,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Target Format."]
#[doc = ""]
pub enum Format {
    #[serde(rename = "qcow2")]
    Qcow2,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "vmdk")]
    Vmdk,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "The config key the disk will be moved to on the target VM (for example, ide0 or scsi1). Default is the source disk key."]
#[doc = ""]
pub enum TargetDisk {
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
    #[serde(rename = "unused0")]
    Unused0,
    #[serde(rename = "unused1")]
    Unused1,
    #[serde(rename = "unused10")]
    Unused10,
    #[serde(rename = "unused100")]
    Unused100,
    #[serde(rename = "unused101")]
    Unused101,
    #[serde(rename = "unused102")]
    Unused102,
    #[serde(rename = "unused103")]
    Unused103,
    #[serde(rename = "unused104")]
    Unused104,
    #[serde(rename = "unused105")]
    Unused105,
    #[serde(rename = "unused106")]
    Unused106,
    #[serde(rename = "unused107")]
    Unused107,
    #[serde(rename = "unused108")]
    Unused108,
    #[serde(rename = "unused109")]
    Unused109,
    #[serde(rename = "unused11")]
    Unused11,
    #[serde(rename = "unused110")]
    Unused110,
    #[serde(rename = "unused111")]
    Unused111,
    #[serde(rename = "unused112")]
    Unused112,
    #[serde(rename = "unused113")]
    Unused113,
    #[serde(rename = "unused114")]
    Unused114,
    #[serde(rename = "unused115")]
    Unused115,
    #[serde(rename = "unused116")]
    Unused116,
    #[serde(rename = "unused117")]
    Unused117,
    #[serde(rename = "unused118")]
    Unused118,
    #[serde(rename = "unused119")]
    Unused119,
    #[serde(rename = "unused12")]
    Unused12,
    #[serde(rename = "unused120")]
    Unused120,
    #[serde(rename = "unused121")]
    Unused121,
    #[serde(rename = "unused122")]
    Unused122,
    #[serde(rename = "unused123")]
    Unused123,
    #[serde(rename = "unused124")]
    Unused124,
    #[serde(rename = "unused125")]
    Unused125,
    #[serde(rename = "unused126")]
    Unused126,
    #[serde(rename = "unused127")]
    Unused127,
    #[serde(rename = "unused128")]
    Unused128,
    #[serde(rename = "unused129")]
    Unused129,
    #[serde(rename = "unused13")]
    Unused13,
    #[serde(rename = "unused130")]
    Unused130,
    #[serde(rename = "unused131")]
    Unused131,
    #[serde(rename = "unused132")]
    Unused132,
    #[serde(rename = "unused133")]
    Unused133,
    #[serde(rename = "unused134")]
    Unused134,
    #[serde(rename = "unused135")]
    Unused135,
    #[serde(rename = "unused136")]
    Unused136,
    #[serde(rename = "unused137")]
    Unused137,
    #[serde(rename = "unused138")]
    Unused138,
    #[serde(rename = "unused139")]
    Unused139,
    #[serde(rename = "unused14")]
    Unused14,
    #[serde(rename = "unused140")]
    Unused140,
    #[serde(rename = "unused141")]
    Unused141,
    #[serde(rename = "unused142")]
    Unused142,
    #[serde(rename = "unused143")]
    Unused143,
    #[serde(rename = "unused144")]
    Unused144,
    #[serde(rename = "unused145")]
    Unused145,
    #[serde(rename = "unused146")]
    Unused146,
    #[serde(rename = "unused147")]
    Unused147,
    #[serde(rename = "unused148")]
    Unused148,
    #[serde(rename = "unused149")]
    Unused149,
    #[serde(rename = "unused15")]
    Unused15,
    #[serde(rename = "unused150")]
    Unused150,
    #[serde(rename = "unused151")]
    Unused151,
    #[serde(rename = "unused152")]
    Unused152,
    #[serde(rename = "unused153")]
    Unused153,
    #[serde(rename = "unused154")]
    Unused154,
    #[serde(rename = "unused155")]
    Unused155,
    #[serde(rename = "unused156")]
    Unused156,
    #[serde(rename = "unused157")]
    Unused157,
    #[serde(rename = "unused158")]
    Unused158,
    #[serde(rename = "unused159")]
    Unused159,
    #[serde(rename = "unused16")]
    Unused16,
    #[serde(rename = "unused160")]
    Unused160,
    #[serde(rename = "unused161")]
    Unused161,
    #[serde(rename = "unused162")]
    Unused162,
    #[serde(rename = "unused163")]
    Unused163,
    #[serde(rename = "unused164")]
    Unused164,
    #[serde(rename = "unused165")]
    Unused165,
    #[serde(rename = "unused166")]
    Unused166,
    #[serde(rename = "unused167")]
    Unused167,
    #[serde(rename = "unused168")]
    Unused168,
    #[serde(rename = "unused169")]
    Unused169,
    #[serde(rename = "unused17")]
    Unused17,
    #[serde(rename = "unused170")]
    Unused170,
    #[serde(rename = "unused171")]
    Unused171,
    #[serde(rename = "unused172")]
    Unused172,
    #[serde(rename = "unused173")]
    Unused173,
    #[serde(rename = "unused174")]
    Unused174,
    #[serde(rename = "unused175")]
    Unused175,
    #[serde(rename = "unused176")]
    Unused176,
    #[serde(rename = "unused177")]
    Unused177,
    #[serde(rename = "unused178")]
    Unused178,
    #[serde(rename = "unused179")]
    Unused179,
    #[serde(rename = "unused18")]
    Unused18,
    #[serde(rename = "unused180")]
    Unused180,
    #[serde(rename = "unused181")]
    Unused181,
    #[serde(rename = "unused182")]
    Unused182,
    #[serde(rename = "unused183")]
    Unused183,
    #[serde(rename = "unused184")]
    Unused184,
    #[serde(rename = "unused185")]
    Unused185,
    #[serde(rename = "unused186")]
    Unused186,
    #[serde(rename = "unused187")]
    Unused187,
    #[serde(rename = "unused188")]
    Unused188,
    #[serde(rename = "unused189")]
    Unused189,
    #[serde(rename = "unused19")]
    Unused19,
    #[serde(rename = "unused190")]
    Unused190,
    #[serde(rename = "unused191")]
    Unused191,
    #[serde(rename = "unused192")]
    Unused192,
    #[serde(rename = "unused193")]
    Unused193,
    #[serde(rename = "unused194")]
    Unused194,
    #[serde(rename = "unused195")]
    Unused195,
    #[serde(rename = "unused196")]
    Unused196,
    #[serde(rename = "unused197")]
    Unused197,
    #[serde(rename = "unused198")]
    Unused198,
    #[serde(rename = "unused199")]
    Unused199,
    #[serde(rename = "unused2")]
    Unused2,
    #[serde(rename = "unused20")]
    Unused20,
    #[serde(rename = "unused200")]
    Unused200,
    #[serde(rename = "unused201")]
    Unused201,
    #[serde(rename = "unused202")]
    Unused202,
    #[serde(rename = "unused203")]
    Unused203,
    #[serde(rename = "unused204")]
    Unused204,
    #[serde(rename = "unused205")]
    Unused205,
    #[serde(rename = "unused206")]
    Unused206,
    #[serde(rename = "unused207")]
    Unused207,
    #[serde(rename = "unused208")]
    Unused208,
    #[serde(rename = "unused209")]
    Unused209,
    #[serde(rename = "unused21")]
    Unused21,
    #[serde(rename = "unused210")]
    Unused210,
    #[serde(rename = "unused211")]
    Unused211,
    #[serde(rename = "unused212")]
    Unused212,
    #[serde(rename = "unused213")]
    Unused213,
    #[serde(rename = "unused214")]
    Unused214,
    #[serde(rename = "unused215")]
    Unused215,
    #[serde(rename = "unused216")]
    Unused216,
    #[serde(rename = "unused217")]
    Unused217,
    #[serde(rename = "unused218")]
    Unused218,
    #[serde(rename = "unused219")]
    Unused219,
    #[serde(rename = "unused22")]
    Unused22,
    #[serde(rename = "unused220")]
    Unused220,
    #[serde(rename = "unused221")]
    Unused221,
    #[serde(rename = "unused222")]
    Unused222,
    #[serde(rename = "unused223")]
    Unused223,
    #[serde(rename = "unused224")]
    Unused224,
    #[serde(rename = "unused225")]
    Unused225,
    #[serde(rename = "unused226")]
    Unused226,
    #[serde(rename = "unused227")]
    Unused227,
    #[serde(rename = "unused228")]
    Unused228,
    #[serde(rename = "unused229")]
    Unused229,
    #[serde(rename = "unused23")]
    Unused23,
    #[serde(rename = "unused230")]
    Unused230,
    #[serde(rename = "unused231")]
    Unused231,
    #[serde(rename = "unused232")]
    Unused232,
    #[serde(rename = "unused233")]
    Unused233,
    #[serde(rename = "unused234")]
    Unused234,
    #[serde(rename = "unused235")]
    Unused235,
    #[serde(rename = "unused236")]
    Unused236,
    #[serde(rename = "unused237")]
    Unused237,
    #[serde(rename = "unused238")]
    Unused238,
    #[serde(rename = "unused239")]
    Unused239,
    #[serde(rename = "unused24")]
    Unused24,
    #[serde(rename = "unused240")]
    Unused240,
    #[serde(rename = "unused241")]
    Unused241,
    #[serde(rename = "unused242")]
    Unused242,
    #[serde(rename = "unused243")]
    Unused243,
    #[serde(rename = "unused244")]
    Unused244,
    #[serde(rename = "unused245")]
    Unused245,
    #[serde(rename = "unused246")]
    Unused246,
    #[serde(rename = "unused247")]
    Unused247,
    #[serde(rename = "unused248")]
    Unused248,
    #[serde(rename = "unused249")]
    Unused249,
    #[serde(rename = "unused25")]
    Unused25,
    #[serde(rename = "unused250")]
    Unused250,
    #[serde(rename = "unused251")]
    Unused251,
    #[serde(rename = "unused252")]
    Unused252,
    #[serde(rename = "unused253")]
    Unused253,
    #[serde(rename = "unused254")]
    Unused254,
    #[serde(rename = "unused255")]
    Unused255,
    #[serde(rename = "unused26")]
    Unused26,
    #[serde(rename = "unused27")]
    Unused27,
    #[serde(rename = "unused28")]
    Unused28,
    #[serde(rename = "unused29")]
    Unused29,
    #[serde(rename = "unused3")]
    Unused3,
    #[serde(rename = "unused30")]
    Unused30,
    #[serde(rename = "unused31")]
    Unused31,
    #[serde(rename = "unused32")]
    Unused32,
    #[serde(rename = "unused33")]
    Unused33,
    #[serde(rename = "unused34")]
    Unused34,
    #[serde(rename = "unused35")]
    Unused35,
    #[serde(rename = "unused36")]
    Unused36,
    #[serde(rename = "unused37")]
    Unused37,
    #[serde(rename = "unused38")]
    Unused38,
    #[serde(rename = "unused39")]
    Unused39,
    #[serde(rename = "unused4")]
    Unused4,
    #[serde(rename = "unused40")]
    Unused40,
    #[serde(rename = "unused41")]
    Unused41,
    #[serde(rename = "unused42")]
    Unused42,
    #[serde(rename = "unused43")]
    Unused43,
    #[serde(rename = "unused44")]
    Unused44,
    #[serde(rename = "unused45")]
    Unused45,
    #[serde(rename = "unused46")]
    Unused46,
    #[serde(rename = "unused47")]
    Unused47,
    #[serde(rename = "unused48")]
    Unused48,
    #[serde(rename = "unused49")]
    Unused49,
    #[serde(rename = "unused5")]
    Unused5,
    #[serde(rename = "unused50")]
    Unused50,
    #[serde(rename = "unused51")]
    Unused51,
    #[serde(rename = "unused52")]
    Unused52,
    #[serde(rename = "unused53")]
    Unused53,
    #[serde(rename = "unused54")]
    Unused54,
    #[serde(rename = "unused55")]
    Unused55,
    #[serde(rename = "unused56")]
    Unused56,
    #[serde(rename = "unused57")]
    Unused57,
    #[serde(rename = "unused58")]
    Unused58,
    #[serde(rename = "unused59")]
    Unused59,
    #[serde(rename = "unused6")]
    Unused6,
    #[serde(rename = "unused60")]
    Unused60,
    #[serde(rename = "unused61")]
    Unused61,
    #[serde(rename = "unused62")]
    Unused62,
    #[serde(rename = "unused63")]
    Unused63,
    #[serde(rename = "unused64")]
    Unused64,
    #[serde(rename = "unused65")]
    Unused65,
    #[serde(rename = "unused66")]
    Unused66,
    #[serde(rename = "unused67")]
    Unused67,
    #[serde(rename = "unused68")]
    Unused68,
    #[serde(rename = "unused69")]
    Unused69,
    #[serde(rename = "unused7")]
    Unused7,
    #[serde(rename = "unused70")]
    Unused70,
    #[serde(rename = "unused71")]
    Unused71,
    #[serde(rename = "unused72")]
    Unused72,
    #[serde(rename = "unused73")]
    Unused73,
    #[serde(rename = "unused74")]
    Unused74,
    #[serde(rename = "unused75")]
    Unused75,
    #[serde(rename = "unused76")]
    Unused76,
    #[serde(rename = "unused77")]
    Unused77,
    #[serde(rename = "unused78")]
    Unused78,
    #[serde(rename = "unused79")]
    Unused79,
    #[serde(rename = "unused8")]
    Unused8,
    #[serde(rename = "unused80")]
    Unused80,
    #[serde(rename = "unused81")]
    Unused81,
    #[serde(rename = "unused82")]
    Unused82,
    #[serde(rename = "unused83")]
    Unused83,
    #[serde(rename = "unused84")]
    Unused84,
    #[serde(rename = "unused85")]
    Unused85,
    #[serde(rename = "unused86")]
    Unused86,
    #[serde(rename = "unused87")]
    Unused87,
    #[serde(rename = "unused88")]
    Unused88,
    #[serde(rename = "unused89")]
    Unused89,
    #[serde(rename = "unused9")]
    Unused9,
    #[serde(rename = "unused90")]
    Unused90,
    #[serde(rename = "unused91")]
    Unused91,
    #[serde(rename = "unused92")]
    Unused92,
    #[serde(rename = "unused93")]
    Unused93,
    #[serde(rename = "unused94")]
    Unused94,
    #[serde(rename = "unused95")]
    Unused95,
    #[serde(rename = "unused96")]
    Unused96,
    #[serde(rename = "unused97")]
    Unused97,
    #[serde(rename = "unused98")]
    Unused98,
    #[serde(rename = "unused99")]
    Unused99,
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
