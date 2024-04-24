#[derive(Debug, Clone)]
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
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<u64>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
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
            "unused0" => Ok(Self::Unused0),
            "unused1" => Ok(Self::Unused1),
            "unused10" => Ok(Self::Unused10),
            "unused100" => Ok(Self::Unused100),
            "unused101" => Ok(Self::Unused101),
            "unused102" => Ok(Self::Unused102),
            "unused103" => Ok(Self::Unused103),
            "unused104" => Ok(Self::Unused104),
            "unused105" => Ok(Self::Unused105),
            "unused106" => Ok(Self::Unused106),
            "unused107" => Ok(Self::Unused107),
            "unused108" => Ok(Self::Unused108),
            "unused109" => Ok(Self::Unused109),
            "unused11" => Ok(Self::Unused11),
            "unused110" => Ok(Self::Unused110),
            "unused111" => Ok(Self::Unused111),
            "unused112" => Ok(Self::Unused112),
            "unused113" => Ok(Self::Unused113),
            "unused114" => Ok(Self::Unused114),
            "unused115" => Ok(Self::Unused115),
            "unused116" => Ok(Self::Unused116),
            "unused117" => Ok(Self::Unused117),
            "unused118" => Ok(Self::Unused118),
            "unused119" => Ok(Self::Unused119),
            "unused12" => Ok(Self::Unused12),
            "unused120" => Ok(Self::Unused120),
            "unused121" => Ok(Self::Unused121),
            "unused122" => Ok(Self::Unused122),
            "unused123" => Ok(Self::Unused123),
            "unused124" => Ok(Self::Unused124),
            "unused125" => Ok(Self::Unused125),
            "unused126" => Ok(Self::Unused126),
            "unused127" => Ok(Self::Unused127),
            "unused128" => Ok(Self::Unused128),
            "unused129" => Ok(Self::Unused129),
            "unused13" => Ok(Self::Unused13),
            "unused130" => Ok(Self::Unused130),
            "unused131" => Ok(Self::Unused131),
            "unused132" => Ok(Self::Unused132),
            "unused133" => Ok(Self::Unused133),
            "unused134" => Ok(Self::Unused134),
            "unused135" => Ok(Self::Unused135),
            "unused136" => Ok(Self::Unused136),
            "unused137" => Ok(Self::Unused137),
            "unused138" => Ok(Self::Unused138),
            "unused139" => Ok(Self::Unused139),
            "unused14" => Ok(Self::Unused14),
            "unused140" => Ok(Self::Unused140),
            "unused141" => Ok(Self::Unused141),
            "unused142" => Ok(Self::Unused142),
            "unused143" => Ok(Self::Unused143),
            "unused144" => Ok(Self::Unused144),
            "unused145" => Ok(Self::Unused145),
            "unused146" => Ok(Self::Unused146),
            "unused147" => Ok(Self::Unused147),
            "unused148" => Ok(Self::Unused148),
            "unused149" => Ok(Self::Unused149),
            "unused15" => Ok(Self::Unused15),
            "unused150" => Ok(Self::Unused150),
            "unused151" => Ok(Self::Unused151),
            "unused152" => Ok(Self::Unused152),
            "unused153" => Ok(Self::Unused153),
            "unused154" => Ok(Self::Unused154),
            "unused155" => Ok(Self::Unused155),
            "unused156" => Ok(Self::Unused156),
            "unused157" => Ok(Self::Unused157),
            "unused158" => Ok(Self::Unused158),
            "unused159" => Ok(Self::Unused159),
            "unused16" => Ok(Self::Unused16),
            "unused160" => Ok(Self::Unused160),
            "unused161" => Ok(Self::Unused161),
            "unused162" => Ok(Self::Unused162),
            "unused163" => Ok(Self::Unused163),
            "unused164" => Ok(Self::Unused164),
            "unused165" => Ok(Self::Unused165),
            "unused166" => Ok(Self::Unused166),
            "unused167" => Ok(Self::Unused167),
            "unused168" => Ok(Self::Unused168),
            "unused169" => Ok(Self::Unused169),
            "unused17" => Ok(Self::Unused17),
            "unused170" => Ok(Self::Unused170),
            "unused171" => Ok(Self::Unused171),
            "unused172" => Ok(Self::Unused172),
            "unused173" => Ok(Self::Unused173),
            "unused174" => Ok(Self::Unused174),
            "unused175" => Ok(Self::Unused175),
            "unused176" => Ok(Self::Unused176),
            "unused177" => Ok(Self::Unused177),
            "unused178" => Ok(Self::Unused178),
            "unused179" => Ok(Self::Unused179),
            "unused18" => Ok(Self::Unused18),
            "unused180" => Ok(Self::Unused180),
            "unused181" => Ok(Self::Unused181),
            "unused182" => Ok(Self::Unused182),
            "unused183" => Ok(Self::Unused183),
            "unused184" => Ok(Self::Unused184),
            "unused185" => Ok(Self::Unused185),
            "unused186" => Ok(Self::Unused186),
            "unused187" => Ok(Self::Unused187),
            "unused188" => Ok(Self::Unused188),
            "unused189" => Ok(Self::Unused189),
            "unused19" => Ok(Self::Unused19),
            "unused190" => Ok(Self::Unused190),
            "unused191" => Ok(Self::Unused191),
            "unused192" => Ok(Self::Unused192),
            "unused193" => Ok(Self::Unused193),
            "unused194" => Ok(Self::Unused194),
            "unused195" => Ok(Self::Unused195),
            "unused196" => Ok(Self::Unused196),
            "unused197" => Ok(Self::Unused197),
            "unused198" => Ok(Self::Unused198),
            "unused199" => Ok(Self::Unused199),
            "unused2" => Ok(Self::Unused2),
            "unused20" => Ok(Self::Unused20),
            "unused200" => Ok(Self::Unused200),
            "unused201" => Ok(Self::Unused201),
            "unused202" => Ok(Self::Unused202),
            "unused203" => Ok(Self::Unused203),
            "unused204" => Ok(Self::Unused204),
            "unused205" => Ok(Self::Unused205),
            "unused206" => Ok(Self::Unused206),
            "unused207" => Ok(Self::Unused207),
            "unused208" => Ok(Self::Unused208),
            "unused209" => Ok(Self::Unused209),
            "unused21" => Ok(Self::Unused21),
            "unused210" => Ok(Self::Unused210),
            "unused211" => Ok(Self::Unused211),
            "unused212" => Ok(Self::Unused212),
            "unused213" => Ok(Self::Unused213),
            "unused214" => Ok(Self::Unused214),
            "unused215" => Ok(Self::Unused215),
            "unused216" => Ok(Self::Unused216),
            "unused217" => Ok(Self::Unused217),
            "unused218" => Ok(Self::Unused218),
            "unused219" => Ok(Self::Unused219),
            "unused22" => Ok(Self::Unused22),
            "unused220" => Ok(Self::Unused220),
            "unused221" => Ok(Self::Unused221),
            "unused222" => Ok(Self::Unused222),
            "unused223" => Ok(Self::Unused223),
            "unused224" => Ok(Self::Unused224),
            "unused225" => Ok(Self::Unused225),
            "unused226" => Ok(Self::Unused226),
            "unused227" => Ok(Self::Unused227),
            "unused228" => Ok(Self::Unused228),
            "unused229" => Ok(Self::Unused229),
            "unused23" => Ok(Self::Unused23),
            "unused230" => Ok(Self::Unused230),
            "unused231" => Ok(Self::Unused231),
            "unused232" => Ok(Self::Unused232),
            "unused233" => Ok(Self::Unused233),
            "unused234" => Ok(Self::Unused234),
            "unused235" => Ok(Self::Unused235),
            "unused236" => Ok(Self::Unused236),
            "unused237" => Ok(Self::Unused237),
            "unused238" => Ok(Self::Unused238),
            "unused239" => Ok(Self::Unused239),
            "unused24" => Ok(Self::Unused24),
            "unused240" => Ok(Self::Unused240),
            "unused241" => Ok(Self::Unused241),
            "unused242" => Ok(Self::Unused242),
            "unused243" => Ok(Self::Unused243),
            "unused244" => Ok(Self::Unused244),
            "unused245" => Ok(Self::Unused245),
            "unused246" => Ok(Self::Unused246),
            "unused247" => Ok(Self::Unused247),
            "unused248" => Ok(Self::Unused248),
            "unused249" => Ok(Self::Unused249),
            "unused25" => Ok(Self::Unused25),
            "unused250" => Ok(Self::Unused250),
            "unused251" => Ok(Self::Unused251),
            "unused252" => Ok(Self::Unused252),
            "unused253" => Ok(Self::Unused253),
            "unused254" => Ok(Self::Unused254),
            "unused255" => Ok(Self::Unused255),
            "unused26" => Ok(Self::Unused26),
            "unused27" => Ok(Self::Unused27),
            "unused28" => Ok(Self::Unused28),
            "unused29" => Ok(Self::Unused29),
            "unused3" => Ok(Self::Unused3),
            "unused30" => Ok(Self::Unused30),
            "unused31" => Ok(Self::Unused31),
            "unused32" => Ok(Self::Unused32),
            "unused33" => Ok(Self::Unused33),
            "unused34" => Ok(Self::Unused34),
            "unused35" => Ok(Self::Unused35),
            "unused36" => Ok(Self::Unused36),
            "unused37" => Ok(Self::Unused37),
            "unused38" => Ok(Self::Unused38),
            "unused39" => Ok(Self::Unused39),
            "unused4" => Ok(Self::Unused4),
            "unused40" => Ok(Self::Unused40),
            "unused41" => Ok(Self::Unused41),
            "unused42" => Ok(Self::Unused42),
            "unused43" => Ok(Self::Unused43),
            "unused44" => Ok(Self::Unused44),
            "unused45" => Ok(Self::Unused45),
            "unused46" => Ok(Self::Unused46),
            "unused47" => Ok(Self::Unused47),
            "unused48" => Ok(Self::Unused48),
            "unused49" => Ok(Self::Unused49),
            "unused5" => Ok(Self::Unused5),
            "unused50" => Ok(Self::Unused50),
            "unused51" => Ok(Self::Unused51),
            "unused52" => Ok(Self::Unused52),
            "unused53" => Ok(Self::Unused53),
            "unused54" => Ok(Self::Unused54),
            "unused55" => Ok(Self::Unused55),
            "unused56" => Ok(Self::Unused56),
            "unused57" => Ok(Self::Unused57),
            "unused58" => Ok(Self::Unused58),
            "unused59" => Ok(Self::Unused59),
            "unused6" => Ok(Self::Unused6),
            "unused60" => Ok(Self::Unused60),
            "unused61" => Ok(Self::Unused61),
            "unused62" => Ok(Self::Unused62),
            "unused63" => Ok(Self::Unused63),
            "unused64" => Ok(Self::Unused64),
            "unused65" => Ok(Self::Unused65),
            "unused66" => Ok(Self::Unused66),
            "unused67" => Ok(Self::Unused67),
            "unused68" => Ok(Self::Unused68),
            "unused69" => Ok(Self::Unused69),
            "unused7" => Ok(Self::Unused7),
            "unused70" => Ok(Self::Unused70),
            "unused71" => Ok(Self::Unused71),
            "unused72" => Ok(Self::Unused72),
            "unused73" => Ok(Self::Unused73),
            "unused74" => Ok(Self::Unused74),
            "unused75" => Ok(Self::Unused75),
            "unused76" => Ok(Self::Unused76),
            "unused77" => Ok(Self::Unused77),
            "unused78" => Ok(Self::Unused78),
            "unused79" => Ok(Self::Unused79),
            "unused8" => Ok(Self::Unused8),
            "unused80" => Ok(Self::Unused80),
            "unused81" => Ok(Self::Unused81),
            "unused82" => Ok(Self::Unused82),
            "unused83" => Ok(Self::Unused83),
            "unused84" => Ok(Self::Unused84),
            "unused85" => Ok(Self::Unused85),
            "unused86" => Ok(Self::Unused86),
            "unused87" => Ok(Self::Unused87),
            "unused88" => Ok(Self::Unused88),
            "unused89" => Ok(Self::Unused89),
            "unused9" => Ok(Self::Unused9),
            "unused90" => Ok(Self::Unused90),
            "unused91" => Ok(Self::Unused91),
            "unused92" => Ok(Self::Unused92),
            "unused93" => Ok(Self::Unused93),
            "unused94" => Ok(Self::Unused94),
            "unused95" => Ok(Self::Unused95),
            "unused96" => Ok(Self::Unused96),
            "unused97" => Ok(Self::Unused97),
            "unused98" => Ok(Self::Unused98),
            "unused99" => Ok(Self::Unused99),
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
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
impl TryFrom<&str> for Format {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "qcow2" => Ok(Self::Qcow2),
            "raw" => Ok(Self::Raw),
            "vmdk" => Ok(Self::Vmdk),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
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
impl TryFrom<&str> for TargetDisk {
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
            "unused0" => Ok(Self::Unused0),
            "unused1" => Ok(Self::Unused1),
            "unused10" => Ok(Self::Unused10),
            "unused100" => Ok(Self::Unused100),
            "unused101" => Ok(Self::Unused101),
            "unused102" => Ok(Self::Unused102),
            "unused103" => Ok(Self::Unused103),
            "unused104" => Ok(Self::Unused104),
            "unused105" => Ok(Self::Unused105),
            "unused106" => Ok(Self::Unused106),
            "unused107" => Ok(Self::Unused107),
            "unused108" => Ok(Self::Unused108),
            "unused109" => Ok(Self::Unused109),
            "unused11" => Ok(Self::Unused11),
            "unused110" => Ok(Self::Unused110),
            "unused111" => Ok(Self::Unused111),
            "unused112" => Ok(Self::Unused112),
            "unused113" => Ok(Self::Unused113),
            "unused114" => Ok(Self::Unused114),
            "unused115" => Ok(Self::Unused115),
            "unused116" => Ok(Self::Unused116),
            "unused117" => Ok(Self::Unused117),
            "unused118" => Ok(Self::Unused118),
            "unused119" => Ok(Self::Unused119),
            "unused12" => Ok(Self::Unused12),
            "unused120" => Ok(Self::Unused120),
            "unused121" => Ok(Self::Unused121),
            "unused122" => Ok(Self::Unused122),
            "unused123" => Ok(Self::Unused123),
            "unused124" => Ok(Self::Unused124),
            "unused125" => Ok(Self::Unused125),
            "unused126" => Ok(Self::Unused126),
            "unused127" => Ok(Self::Unused127),
            "unused128" => Ok(Self::Unused128),
            "unused129" => Ok(Self::Unused129),
            "unused13" => Ok(Self::Unused13),
            "unused130" => Ok(Self::Unused130),
            "unused131" => Ok(Self::Unused131),
            "unused132" => Ok(Self::Unused132),
            "unused133" => Ok(Self::Unused133),
            "unused134" => Ok(Self::Unused134),
            "unused135" => Ok(Self::Unused135),
            "unused136" => Ok(Self::Unused136),
            "unused137" => Ok(Self::Unused137),
            "unused138" => Ok(Self::Unused138),
            "unused139" => Ok(Self::Unused139),
            "unused14" => Ok(Self::Unused14),
            "unused140" => Ok(Self::Unused140),
            "unused141" => Ok(Self::Unused141),
            "unused142" => Ok(Self::Unused142),
            "unused143" => Ok(Self::Unused143),
            "unused144" => Ok(Self::Unused144),
            "unused145" => Ok(Self::Unused145),
            "unused146" => Ok(Self::Unused146),
            "unused147" => Ok(Self::Unused147),
            "unused148" => Ok(Self::Unused148),
            "unused149" => Ok(Self::Unused149),
            "unused15" => Ok(Self::Unused15),
            "unused150" => Ok(Self::Unused150),
            "unused151" => Ok(Self::Unused151),
            "unused152" => Ok(Self::Unused152),
            "unused153" => Ok(Self::Unused153),
            "unused154" => Ok(Self::Unused154),
            "unused155" => Ok(Self::Unused155),
            "unused156" => Ok(Self::Unused156),
            "unused157" => Ok(Self::Unused157),
            "unused158" => Ok(Self::Unused158),
            "unused159" => Ok(Self::Unused159),
            "unused16" => Ok(Self::Unused16),
            "unused160" => Ok(Self::Unused160),
            "unused161" => Ok(Self::Unused161),
            "unused162" => Ok(Self::Unused162),
            "unused163" => Ok(Self::Unused163),
            "unused164" => Ok(Self::Unused164),
            "unused165" => Ok(Self::Unused165),
            "unused166" => Ok(Self::Unused166),
            "unused167" => Ok(Self::Unused167),
            "unused168" => Ok(Self::Unused168),
            "unused169" => Ok(Self::Unused169),
            "unused17" => Ok(Self::Unused17),
            "unused170" => Ok(Self::Unused170),
            "unused171" => Ok(Self::Unused171),
            "unused172" => Ok(Self::Unused172),
            "unused173" => Ok(Self::Unused173),
            "unused174" => Ok(Self::Unused174),
            "unused175" => Ok(Self::Unused175),
            "unused176" => Ok(Self::Unused176),
            "unused177" => Ok(Self::Unused177),
            "unused178" => Ok(Self::Unused178),
            "unused179" => Ok(Self::Unused179),
            "unused18" => Ok(Self::Unused18),
            "unused180" => Ok(Self::Unused180),
            "unused181" => Ok(Self::Unused181),
            "unused182" => Ok(Self::Unused182),
            "unused183" => Ok(Self::Unused183),
            "unused184" => Ok(Self::Unused184),
            "unused185" => Ok(Self::Unused185),
            "unused186" => Ok(Self::Unused186),
            "unused187" => Ok(Self::Unused187),
            "unused188" => Ok(Self::Unused188),
            "unused189" => Ok(Self::Unused189),
            "unused19" => Ok(Self::Unused19),
            "unused190" => Ok(Self::Unused190),
            "unused191" => Ok(Self::Unused191),
            "unused192" => Ok(Self::Unused192),
            "unused193" => Ok(Self::Unused193),
            "unused194" => Ok(Self::Unused194),
            "unused195" => Ok(Self::Unused195),
            "unused196" => Ok(Self::Unused196),
            "unused197" => Ok(Self::Unused197),
            "unused198" => Ok(Self::Unused198),
            "unused199" => Ok(Self::Unused199),
            "unused2" => Ok(Self::Unused2),
            "unused20" => Ok(Self::Unused20),
            "unused200" => Ok(Self::Unused200),
            "unused201" => Ok(Self::Unused201),
            "unused202" => Ok(Self::Unused202),
            "unused203" => Ok(Self::Unused203),
            "unused204" => Ok(Self::Unused204),
            "unused205" => Ok(Self::Unused205),
            "unused206" => Ok(Self::Unused206),
            "unused207" => Ok(Self::Unused207),
            "unused208" => Ok(Self::Unused208),
            "unused209" => Ok(Self::Unused209),
            "unused21" => Ok(Self::Unused21),
            "unused210" => Ok(Self::Unused210),
            "unused211" => Ok(Self::Unused211),
            "unused212" => Ok(Self::Unused212),
            "unused213" => Ok(Self::Unused213),
            "unused214" => Ok(Self::Unused214),
            "unused215" => Ok(Self::Unused215),
            "unused216" => Ok(Self::Unused216),
            "unused217" => Ok(Self::Unused217),
            "unused218" => Ok(Self::Unused218),
            "unused219" => Ok(Self::Unused219),
            "unused22" => Ok(Self::Unused22),
            "unused220" => Ok(Self::Unused220),
            "unused221" => Ok(Self::Unused221),
            "unused222" => Ok(Self::Unused222),
            "unused223" => Ok(Self::Unused223),
            "unused224" => Ok(Self::Unused224),
            "unused225" => Ok(Self::Unused225),
            "unused226" => Ok(Self::Unused226),
            "unused227" => Ok(Self::Unused227),
            "unused228" => Ok(Self::Unused228),
            "unused229" => Ok(Self::Unused229),
            "unused23" => Ok(Self::Unused23),
            "unused230" => Ok(Self::Unused230),
            "unused231" => Ok(Self::Unused231),
            "unused232" => Ok(Self::Unused232),
            "unused233" => Ok(Self::Unused233),
            "unused234" => Ok(Self::Unused234),
            "unused235" => Ok(Self::Unused235),
            "unused236" => Ok(Self::Unused236),
            "unused237" => Ok(Self::Unused237),
            "unused238" => Ok(Self::Unused238),
            "unused239" => Ok(Self::Unused239),
            "unused24" => Ok(Self::Unused24),
            "unused240" => Ok(Self::Unused240),
            "unused241" => Ok(Self::Unused241),
            "unused242" => Ok(Self::Unused242),
            "unused243" => Ok(Self::Unused243),
            "unused244" => Ok(Self::Unused244),
            "unused245" => Ok(Self::Unused245),
            "unused246" => Ok(Self::Unused246),
            "unused247" => Ok(Self::Unused247),
            "unused248" => Ok(Self::Unused248),
            "unused249" => Ok(Self::Unused249),
            "unused25" => Ok(Self::Unused25),
            "unused250" => Ok(Self::Unused250),
            "unused251" => Ok(Self::Unused251),
            "unused252" => Ok(Self::Unused252),
            "unused253" => Ok(Self::Unused253),
            "unused254" => Ok(Self::Unused254),
            "unused255" => Ok(Self::Unused255),
            "unused26" => Ok(Self::Unused26),
            "unused27" => Ok(Self::Unused27),
            "unused28" => Ok(Self::Unused28),
            "unused29" => Ok(Self::Unused29),
            "unused3" => Ok(Self::Unused3),
            "unused30" => Ok(Self::Unused30),
            "unused31" => Ok(Self::Unused31),
            "unused32" => Ok(Self::Unused32),
            "unused33" => Ok(Self::Unused33),
            "unused34" => Ok(Self::Unused34),
            "unused35" => Ok(Self::Unused35),
            "unused36" => Ok(Self::Unused36),
            "unused37" => Ok(Self::Unused37),
            "unused38" => Ok(Self::Unused38),
            "unused39" => Ok(Self::Unused39),
            "unused4" => Ok(Self::Unused4),
            "unused40" => Ok(Self::Unused40),
            "unused41" => Ok(Self::Unused41),
            "unused42" => Ok(Self::Unused42),
            "unused43" => Ok(Self::Unused43),
            "unused44" => Ok(Self::Unused44),
            "unused45" => Ok(Self::Unused45),
            "unused46" => Ok(Self::Unused46),
            "unused47" => Ok(Self::Unused47),
            "unused48" => Ok(Self::Unused48),
            "unused49" => Ok(Self::Unused49),
            "unused5" => Ok(Self::Unused5),
            "unused50" => Ok(Self::Unused50),
            "unused51" => Ok(Self::Unused51),
            "unused52" => Ok(Self::Unused52),
            "unused53" => Ok(Self::Unused53),
            "unused54" => Ok(Self::Unused54),
            "unused55" => Ok(Self::Unused55),
            "unused56" => Ok(Self::Unused56),
            "unused57" => Ok(Self::Unused57),
            "unused58" => Ok(Self::Unused58),
            "unused59" => Ok(Self::Unused59),
            "unused6" => Ok(Self::Unused6),
            "unused60" => Ok(Self::Unused60),
            "unused61" => Ok(Self::Unused61),
            "unused62" => Ok(Self::Unused62),
            "unused63" => Ok(Self::Unused63),
            "unused64" => Ok(Self::Unused64),
            "unused65" => Ok(Self::Unused65),
            "unused66" => Ok(Self::Unused66),
            "unused67" => Ok(Self::Unused67),
            "unused68" => Ok(Self::Unused68),
            "unused69" => Ok(Self::Unused69),
            "unused7" => Ok(Self::Unused7),
            "unused70" => Ok(Self::Unused70),
            "unused71" => Ok(Self::Unused71),
            "unused72" => Ok(Self::Unused72),
            "unused73" => Ok(Self::Unused73),
            "unused74" => Ok(Self::Unused74),
            "unused75" => Ok(Self::Unused75),
            "unused76" => Ok(Self::Unused76),
            "unused77" => Ok(Self::Unused77),
            "unused78" => Ok(Self::Unused78),
            "unused79" => Ok(Self::Unused79),
            "unused8" => Ok(Self::Unused8),
            "unused80" => Ok(Self::Unused80),
            "unused81" => Ok(Self::Unused81),
            "unused82" => Ok(Self::Unused82),
            "unused83" => Ok(Self::Unused83),
            "unused84" => Ok(Self::Unused84),
            "unused85" => Ok(Self::Unused85),
            "unused86" => Ok(Self::Unused86),
            "unused87" => Ok(Self::Unused87),
            "unused88" => Ok(Self::Unused88),
            "unused89" => Ok(Self::Unused89),
            "unused9" => Ok(Self::Unused9),
            "unused90" => Ok(Self::Unused90),
            "unused91" => Ok(Self::Unused91),
            "unused92" => Ok(Self::Unused92),
            "unused93" => Ok(Self::Unused93),
            "unused94" => Ok(Self::Unused94),
            "unused95" => Ok(Self::Unused95),
            "unused96" => Ok(Self::Unused96),
            "unused97" => Ok(Self::Unused97),
            "unused98" => Ok(Self::Unused98),
            "unused99" => Ok(Self::Unused99),
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
