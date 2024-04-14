pub struct ResizeClient<T> {
    client: T,
    path: String,
}
impl<T> ResizeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/resize"),
        }
    }
}
impl<T> ResizeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Resize a container mount point."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl PutParams {
    pub fn new(disk: Disk, size: String) -> Self {
        Self {
            disk,
            size,
            digest: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[doc = "The disk you want to resize."]
    #[doc = ""]
    pub disk: Disk,
    #[doc = "The new size. With the '+' sign the value is added to the actual size of the volume and without it, the value is taken as an absolute one. Shrinking disk size is not supported."]
    #[doc = ""]
    pub size: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "The disk you want to resize."]
#[doc = ""]
pub enum Disk {
    #[serde(rename = "mp0")]
    Mp0,
    #[serde(rename = "mp1")]
    Mp1,
    #[serde(rename = "mp10")]
    Mp10,
    #[serde(rename = "mp100")]
    Mp100,
    #[serde(rename = "mp101")]
    Mp101,
    #[serde(rename = "mp102")]
    Mp102,
    #[serde(rename = "mp103")]
    Mp103,
    #[serde(rename = "mp104")]
    Mp104,
    #[serde(rename = "mp105")]
    Mp105,
    #[serde(rename = "mp106")]
    Mp106,
    #[serde(rename = "mp107")]
    Mp107,
    #[serde(rename = "mp108")]
    Mp108,
    #[serde(rename = "mp109")]
    Mp109,
    #[serde(rename = "mp11")]
    Mp11,
    #[serde(rename = "mp110")]
    Mp110,
    #[serde(rename = "mp111")]
    Mp111,
    #[serde(rename = "mp112")]
    Mp112,
    #[serde(rename = "mp113")]
    Mp113,
    #[serde(rename = "mp114")]
    Mp114,
    #[serde(rename = "mp115")]
    Mp115,
    #[serde(rename = "mp116")]
    Mp116,
    #[serde(rename = "mp117")]
    Mp117,
    #[serde(rename = "mp118")]
    Mp118,
    #[serde(rename = "mp119")]
    Mp119,
    #[serde(rename = "mp12")]
    Mp12,
    #[serde(rename = "mp120")]
    Mp120,
    #[serde(rename = "mp121")]
    Mp121,
    #[serde(rename = "mp122")]
    Mp122,
    #[serde(rename = "mp123")]
    Mp123,
    #[serde(rename = "mp124")]
    Mp124,
    #[serde(rename = "mp125")]
    Mp125,
    #[serde(rename = "mp126")]
    Mp126,
    #[serde(rename = "mp127")]
    Mp127,
    #[serde(rename = "mp128")]
    Mp128,
    #[serde(rename = "mp129")]
    Mp129,
    #[serde(rename = "mp13")]
    Mp13,
    #[serde(rename = "mp130")]
    Mp130,
    #[serde(rename = "mp131")]
    Mp131,
    #[serde(rename = "mp132")]
    Mp132,
    #[serde(rename = "mp133")]
    Mp133,
    #[serde(rename = "mp134")]
    Mp134,
    #[serde(rename = "mp135")]
    Mp135,
    #[serde(rename = "mp136")]
    Mp136,
    #[serde(rename = "mp137")]
    Mp137,
    #[serde(rename = "mp138")]
    Mp138,
    #[serde(rename = "mp139")]
    Mp139,
    #[serde(rename = "mp14")]
    Mp14,
    #[serde(rename = "mp140")]
    Mp140,
    #[serde(rename = "mp141")]
    Mp141,
    #[serde(rename = "mp142")]
    Mp142,
    #[serde(rename = "mp143")]
    Mp143,
    #[serde(rename = "mp144")]
    Mp144,
    #[serde(rename = "mp145")]
    Mp145,
    #[serde(rename = "mp146")]
    Mp146,
    #[serde(rename = "mp147")]
    Mp147,
    #[serde(rename = "mp148")]
    Mp148,
    #[serde(rename = "mp149")]
    Mp149,
    #[serde(rename = "mp15")]
    Mp15,
    #[serde(rename = "mp150")]
    Mp150,
    #[serde(rename = "mp151")]
    Mp151,
    #[serde(rename = "mp152")]
    Mp152,
    #[serde(rename = "mp153")]
    Mp153,
    #[serde(rename = "mp154")]
    Mp154,
    #[serde(rename = "mp155")]
    Mp155,
    #[serde(rename = "mp156")]
    Mp156,
    #[serde(rename = "mp157")]
    Mp157,
    #[serde(rename = "mp158")]
    Mp158,
    #[serde(rename = "mp159")]
    Mp159,
    #[serde(rename = "mp16")]
    Mp16,
    #[serde(rename = "mp160")]
    Mp160,
    #[serde(rename = "mp161")]
    Mp161,
    #[serde(rename = "mp162")]
    Mp162,
    #[serde(rename = "mp163")]
    Mp163,
    #[serde(rename = "mp164")]
    Mp164,
    #[serde(rename = "mp165")]
    Mp165,
    #[serde(rename = "mp166")]
    Mp166,
    #[serde(rename = "mp167")]
    Mp167,
    #[serde(rename = "mp168")]
    Mp168,
    #[serde(rename = "mp169")]
    Mp169,
    #[serde(rename = "mp17")]
    Mp17,
    #[serde(rename = "mp170")]
    Mp170,
    #[serde(rename = "mp171")]
    Mp171,
    #[serde(rename = "mp172")]
    Mp172,
    #[serde(rename = "mp173")]
    Mp173,
    #[serde(rename = "mp174")]
    Mp174,
    #[serde(rename = "mp175")]
    Mp175,
    #[serde(rename = "mp176")]
    Mp176,
    #[serde(rename = "mp177")]
    Mp177,
    #[serde(rename = "mp178")]
    Mp178,
    #[serde(rename = "mp179")]
    Mp179,
    #[serde(rename = "mp18")]
    Mp18,
    #[serde(rename = "mp180")]
    Mp180,
    #[serde(rename = "mp181")]
    Mp181,
    #[serde(rename = "mp182")]
    Mp182,
    #[serde(rename = "mp183")]
    Mp183,
    #[serde(rename = "mp184")]
    Mp184,
    #[serde(rename = "mp185")]
    Mp185,
    #[serde(rename = "mp186")]
    Mp186,
    #[serde(rename = "mp187")]
    Mp187,
    #[serde(rename = "mp188")]
    Mp188,
    #[serde(rename = "mp189")]
    Mp189,
    #[serde(rename = "mp19")]
    Mp19,
    #[serde(rename = "mp190")]
    Mp190,
    #[serde(rename = "mp191")]
    Mp191,
    #[serde(rename = "mp192")]
    Mp192,
    #[serde(rename = "mp193")]
    Mp193,
    #[serde(rename = "mp194")]
    Mp194,
    #[serde(rename = "mp195")]
    Mp195,
    #[serde(rename = "mp196")]
    Mp196,
    #[serde(rename = "mp197")]
    Mp197,
    #[serde(rename = "mp198")]
    Mp198,
    #[serde(rename = "mp199")]
    Mp199,
    #[serde(rename = "mp2")]
    Mp2,
    #[serde(rename = "mp20")]
    Mp20,
    #[serde(rename = "mp200")]
    Mp200,
    #[serde(rename = "mp201")]
    Mp201,
    #[serde(rename = "mp202")]
    Mp202,
    #[serde(rename = "mp203")]
    Mp203,
    #[serde(rename = "mp204")]
    Mp204,
    #[serde(rename = "mp205")]
    Mp205,
    #[serde(rename = "mp206")]
    Mp206,
    #[serde(rename = "mp207")]
    Mp207,
    #[serde(rename = "mp208")]
    Mp208,
    #[serde(rename = "mp209")]
    Mp209,
    #[serde(rename = "mp21")]
    Mp21,
    #[serde(rename = "mp210")]
    Mp210,
    #[serde(rename = "mp211")]
    Mp211,
    #[serde(rename = "mp212")]
    Mp212,
    #[serde(rename = "mp213")]
    Mp213,
    #[serde(rename = "mp214")]
    Mp214,
    #[serde(rename = "mp215")]
    Mp215,
    #[serde(rename = "mp216")]
    Mp216,
    #[serde(rename = "mp217")]
    Mp217,
    #[serde(rename = "mp218")]
    Mp218,
    #[serde(rename = "mp219")]
    Mp219,
    #[serde(rename = "mp22")]
    Mp22,
    #[serde(rename = "mp220")]
    Mp220,
    #[serde(rename = "mp221")]
    Mp221,
    #[serde(rename = "mp222")]
    Mp222,
    #[serde(rename = "mp223")]
    Mp223,
    #[serde(rename = "mp224")]
    Mp224,
    #[serde(rename = "mp225")]
    Mp225,
    #[serde(rename = "mp226")]
    Mp226,
    #[serde(rename = "mp227")]
    Mp227,
    #[serde(rename = "mp228")]
    Mp228,
    #[serde(rename = "mp229")]
    Mp229,
    #[serde(rename = "mp23")]
    Mp23,
    #[serde(rename = "mp230")]
    Mp230,
    #[serde(rename = "mp231")]
    Mp231,
    #[serde(rename = "mp232")]
    Mp232,
    #[serde(rename = "mp233")]
    Mp233,
    #[serde(rename = "mp234")]
    Mp234,
    #[serde(rename = "mp235")]
    Mp235,
    #[serde(rename = "mp236")]
    Mp236,
    #[serde(rename = "mp237")]
    Mp237,
    #[serde(rename = "mp238")]
    Mp238,
    #[serde(rename = "mp239")]
    Mp239,
    #[serde(rename = "mp24")]
    Mp24,
    #[serde(rename = "mp240")]
    Mp240,
    #[serde(rename = "mp241")]
    Mp241,
    #[serde(rename = "mp242")]
    Mp242,
    #[serde(rename = "mp243")]
    Mp243,
    #[serde(rename = "mp244")]
    Mp244,
    #[serde(rename = "mp245")]
    Mp245,
    #[serde(rename = "mp246")]
    Mp246,
    #[serde(rename = "mp247")]
    Mp247,
    #[serde(rename = "mp248")]
    Mp248,
    #[serde(rename = "mp249")]
    Mp249,
    #[serde(rename = "mp25")]
    Mp25,
    #[serde(rename = "mp250")]
    Mp250,
    #[serde(rename = "mp251")]
    Mp251,
    #[serde(rename = "mp252")]
    Mp252,
    #[serde(rename = "mp253")]
    Mp253,
    #[serde(rename = "mp254")]
    Mp254,
    #[serde(rename = "mp255")]
    Mp255,
    #[serde(rename = "mp26")]
    Mp26,
    #[serde(rename = "mp27")]
    Mp27,
    #[serde(rename = "mp28")]
    Mp28,
    #[serde(rename = "mp29")]
    Mp29,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "mp30")]
    Mp30,
    #[serde(rename = "mp31")]
    Mp31,
    #[serde(rename = "mp32")]
    Mp32,
    #[serde(rename = "mp33")]
    Mp33,
    #[serde(rename = "mp34")]
    Mp34,
    #[serde(rename = "mp35")]
    Mp35,
    #[serde(rename = "mp36")]
    Mp36,
    #[serde(rename = "mp37")]
    Mp37,
    #[serde(rename = "mp38")]
    Mp38,
    #[serde(rename = "mp39")]
    Mp39,
    #[serde(rename = "mp4")]
    Mp4,
    #[serde(rename = "mp40")]
    Mp40,
    #[serde(rename = "mp41")]
    Mp41,
    #[serde(rename = "mp42")]
    Mp42,
    #[serde(rename = "mp43")]
    Mp43,
    #[serde(rename = "mp44")]
    Mp44,
    #[serde(rename = "mp45")]
    Mp45,
    #[serde(rename = "mp46")]
    Mp46,
    #[serde(rename = "mp47")]
    Mp47,
    #[serde(rename = "mp48")]
    Mp48,
    #[serde(rename = "mp49")]
    Mp49,
    #[serde(rename = "mp5")]
    Mp5,
    #[serde(rename = "mp50")]
    Mp50,
    #[serde(rename = "mp51")]
    Mp51,
    #[serde(rename = "mp52")]
    Mp52,
    #[serde(rename = "mp53")]
    Mp53,
    #[serde(rename = "mp54")]
    Mp54,
    #[serde(rename = "mp55")]
    Mp55,
    #[serde(rename = "mp56")]
    Mp56,
    #[serde(rename = "mp57")]
    Mp57,
    #[serde(rename = "mp58")]
    Mp58,
    #[serde(rename = "mp59")]
    Mp59,
    #[serde(rename = "mp6")]
    Mp6,
    #[serde(rename = "mp60")]
    Mp60,
    #[serde(rename = "mp61")]
    Mp61,
    #[serde(rename = "mp62")]
    Mp62,
    #[serde(rename = "mp63")]
    Mp63,
    #[serde(rename = "mp64")]
    Mp64,
    #[serde(rename = "mp65")]
    Mp65,
    #[serde(rename = "mp66")]
    Mp66,
    #[serde(rename = "mp67")]
    Mp67,
    #[serde(rename = "mp68")]
    Mp68,
    #[serde(rename = "mp69")]
    Mp69,
    #[serde(rename = "mp7")]
    Mp7,
    #[serde(rename = "mp70")]
    Mp70,
    #[serde(rename = "mp71")]
    Mp71,
    #[serde(rename = "mp72")]
    Mp72,
    #[serde(rename = "mp73")]
    Mp73,
    #[serde(rename = "mp74")]
    Mp74,
    #[serde(rename = "mp75")]
    Mp75,
    #[serde(rename = "mp76")]
    Mp76,
    #[serde(rename = "mp77")]
    Mp77,
    #[serde(rename = "mp78")]
    Mp78,
    #[serde(rename = "mp79")]
    Mp79,
    #[serde(rename = "mp8")]
    Mp8,
    #[serde(rename = "mp80")]
    Mp80,
    #[serde(rename = "mp81")]
    Mp81,
    #[serde(rename = "mp82")]
    Mp82,
    #[serde(rename = "mp83")]
    Mp83,
    #[serde(rename = "mp84")]
    Mp84,
    #[serde(rename = "mp85")]
    Mp85,
    #[serde(rename = "mp86")]
    Mp86,
    #[serde(rename = "mp87")]
    Mp87,
    #[serde(rename = "mp88")]
    Mp88,
    #[serde(rename = "mp89")]
    Mp89,
    #[serde(rename = "mp9")]
    Mp9,
    #[serde(rename = "mp90")]
    Mp90,
    #[serde(rename = "mp91")]
    Mp91,
    #[serde(rename = "mp92")]
    Mp92,
    #[serde(rename = "mp93")]
    Mp93,
    #[serde(rename = "mp94")]
    Mp94,
    #[serde(rename = "mp95")]
    Mp95,
    #[serde(rename = "mp96")]
    Mp96,
    #[serde(rename = "mp97")]
    Mp97,
    #[serde(rename = "mp98")]
    Mp98,
    #[serde(rename = "mp99")]
    Mp99,
    #[serde(rename = "rootfs")]
    Rootfs,
}
impl TryFrom<&str> for Disk {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "mp0" => Ok(Self::Mp0),
            "mp1" => Ok(Self::Mp1),
            "mp10" => Ok(Self::Mp10),
            "mp100" => Ok(Self::Mp100),
            "mp101" => Ok(Self::Mp101),
            "mp102" => Ok(Self::Mp102),
            "mp103" => Ok(Self::Mp103),
            "mp104" => Ok(Self::Mp104),
            "mp105" => Ok(Self::Mp105),
            "mp106" => Ok(Self::Mp106),
            "mp107" => Ok(Self::Mp107),
            "mp108" => Ok(Self::Mp108),
            "mp109" => Ok(Self::Mp109),
            "mp11" => Ok(Self::Mp11),
            "mp110" => Ok(Self::Mp110),
            "mp111" => Ok(Self::Mp111),
            "mp112" => Ok(Self::Mp112),
            "mp113" => Ok(Self::Mp113),
            "mp114" => Ok(Self::Mp114),
            "mp115" => Ok(Self::Mp115),
            "mp116" => Ok(Self::Mp116),
            "mp117" => Ok(Self::Mp117),
            "mp118" => Ok(Self::Mp118),
            "mp119" => Ok(Self::Mp119),
            "mp12" => Ok(Self::Mp12),
            "mp120" => Ok(Self::Mp120),
            "mp121" => Ok(Self::Mp121),
            "mp122" => Ok(Self::Mp122),
            "mp123" => Ok(Self::Mp123),
            "mp124" => Ok(Self::Mp124),
            "mp125" => Ok(Self::Mp125),
            "mp126" => Ok(Self::Mp126),
            "mp127" => Ok(Self::Mp127),
            "mp128" => Ok(Self::Mp128),
            "mp129" => Ok(Self::Mp129),
            "mp13" => Ok(Self::Mp13),
            "mp130" => Ok(Self::Mp130),
            "mp131" => Ok(Self::Mp131),
            "mp132" => Ok(Self::Mp132),
            "mp133" => Ok(Self::Mp133),
            "mp134" => Ok(Self::Mp134),
            "mp135" => Ok(Self::Mp135),
            "mp136" => Ok(Self::Mp136),
            "mp137" => Ok(Self::Mp137),
            "mp138" => Ok(Self::Mp138),
            "mp139" => Ok(Self::Mp139),
            "mp14" => Ok(Self::Mp14),
            "mp140" => Ok(Self::Mp140),
            "mp141" => Ok(Self::Mp141),
            "mp142" => Ok(Self::Mp142),
            "mp143" => Ok(Self::Mp143),
            "mp144" => Ok(Self::Mp144),
            "mp145" => Ok(Self::Mp145),
            "mp146" => Ok(Self::Mp146),
            "mp147" => Ok(Self::Mp147),
            "mp148" => Ok(Self::Mp148),
            "mp149" => Ok(Self::Mp149),
            "mp15" => Ok(Self::Mp15),
            "mp150" => Ok(Self::Mp150),
            "mp151" => Ok(Self::Mp151),
            "mp152" => Ok(Self::Mp152),
            "mp153" => Ok(Self::Mp153),
            "mp154" => Ok(Self::Mp154),
            "mp155" => Ok(Self::Mp155),
            "mp156" => Ok(Self::Mp156),
            "mp157" => Ok(Self::Mp157),
            "mp158" => Ok(Self::Mp158),
            "mp159" => Ok(Self::Mp159),
            "mp16" => Ok(Self::Mp16),
            "mp160" => Ok(Self::Mp160),
            "mp161" => Ok(Self::Mp161),
            "mp162" => Ok(Self::Mp162),
            "mp163" => Ok(Self::Mp163),
            "mp164" => Ok(Self::Mp164),
            "mp165" => Ok(Self::Mp165),
            "mp166" => Ok(Self::Mp166),
            "mp167" => Ok(Self::Mp167),
            "mp168" => Ok(Self::Mp168),
            "mp169" => Ok(Self::Mp169),
            "mp17" => Ok(Self::Mp17),
            "mp170" => Ok(Self::Mp170),
            "mp171" => Ok(Self::Mp171),
            "mp172" => Ok(Self::Mp172),
            "mp173" => Ok(Self::Mp173),
            "mp174" => Ok(Self::Mp174),
            "mp175" => Ok(Self::Mp175),
            "mp176" => Ok(Self::Mp176),
            "mp177" => Ok(Self::Mp177),
            "mp178" => Ok(Self::Mp178),
            "mp179" => Ok(Self::Mp179),
            "mp18" => Ok(Self::Mp18),
            "mp180" => Ok(Self::Mp180),
            "mp181" => Ok(Self::Mp181),
            "mp182" => Ok(Self::Mp182),
            "mp183" => Ok(Self::Mp183),
            "mp184" => Ok(Self::Mp184),
            "mp185" => Ok(Self::Mp185),
            "mp186" => Ok(Self::Mp186),
            "mp187" => Ok(Self::Mp187),
            "mp188" => Ok(Self::Mp188),
            "mp189" => Ok(Self::Mp189),
            "mp19" => Ok(Self::Mp19),
            "mp190" => Ok(Self::Mp190),
            "mp191" => Ok(Self::Mp191),
            "mp192" => Ok(Self::Mp192),
            "mp193" => Ok(Self::Mp193),
            "mp194" => Ok(Self::Mp194),
            "mp195" => Ok(Self::Mp195),
            "mp196" => Ok(Self::Mp196),
            "mp197" => Ok(Self::Mp197),
            "mp198" => Ok(Self::Mp198),
            "mp199" => Ok(Self::Mp199),
            "mp2" => Ok(Self::Mp2),
            "mp20" => Ok(Self::Mp20),
            "mp200" => Ok(Self::Mp200),
            "mp201" => Ok(Self::Mp201),
            "mp202" => Ok(Self::Mp202),
            "mp203" => Ok(Self::Mp203),
            "mp204" => Ok(Self::Mp204),
            "mp205" => Ok(Self::Mp205),
            "mp206" => Ok(Self::Mp206),
            "mp207" => Ok(Self::Mp207),
            "mp208" => Ok(Self::Mp208),
            "mp209" => Ok(Self::Mp209),
            "mp21" => Ok(Self::Mp21),
            "mp210" => Ok(Self::Mp210),
            "mp211" => Ok(Self::Mp211),
            "mp212" => Ok(Self::Mp212),
            "mp213" => Ok(Self::Mp213),
            "mp214" => Ok(Self::Mp214),
            "mp215" => Ok(Self::Mp215),
            "mp216" => Ok(Self::Mp216),
            "mp217" => Ok(Self::Mp217),
            "mp218" => Ok(Self::Mp218),
            "mp219" => Ok(Self::Mp219),
            "mp22" => Ok(Self::Mp22),
            "mp220" => Ok(Self::Mp220),
            "mp221" => Ok(Self::Mp221),
            "mp222" => Ok(Self::Mp222),
            "mp223" => Ok(Self::Mp223),
            "mp224" => Ok(Self::Mp224),
            "mp225" => Ok(Self::Mp225),
            "mp226" => Ok(Self::Mp226),
            "mp227" => Ok(Self::Mp227),
            "mp228" => Ok(Self::Mp228),
            "mp229" => Ok(Self::Mp229),
            "mp23" => Ok(Self::Mp23),
            "mp230" => Ok(Self::Mp230),
            "mp231" => Ok(Self::Mp231),
            "mp232" => Ok(Self::Mp232),
            "mp233" => Ok(Self::Mp233),
            "mp234" => Ok(Self::Mp234),
            "mp235" => Ok(Self::Mp235),
            "mp236" => Ok(Self::Mp236),
            "mp237" => Ok(Self::Mp237),
            "mp238" => Ok(Self::Mp238),
            "mp239" => Ok(Self::Mp239),
            "mp24" => Ok(Self::Mp24),
            "mp240" => Ok(Self::Mp240),
            "mp241" => Ok(Self::Mp241),
            "mp242" => Ok(Self::Mp242),
            "mp243" => Ok(Self::Mp243),
            "mp244" => Ok(Self::Mp244),
            "mp245" => Ok(Self::Mp245),
            "mp246" => Ok(Self::Mp246),
            "mp247" => Ok(Self::Mp247),
            "mp248" => Ok(Self::Mp248),
            "mp249" => Ok(Self::Mp249),
            "mp25" => Ok(Self::Mp25),
            "mp250" => Ok(Self::Mp250),
            "mp251" => Ok(Self::Mp251),
            "mp252" => Ok(Self::Mp252),
            "mp253" => Ok(Self::Mp253),
            "mp254" => Ok(Self::Mp254),
            "mp255" => Ok(Self::Mp255),
            "mp26" => Ok(Self::Mp26),
            "mp27" => Ok(Self::Mp27),
            "mp28" => Ok(Self::Mp28),
            "mp29" => Ok(Self::Mp29),
            "mp3" => Ok(Self::Mp3),
            "mp30" => Ok(Self::Mp30),
            "mp31" => Ok(Self::Mp31),
            "mp32" => Ok(Self::Mp32),
            "mp33" => Ok(Self::Mp33),
            "mp34" => Ok(Self::Mp34),
            "mp35" => Ok(Self::Mp35),
            "mp36" => Ok(Self::Mp36),
            "mp37" => Ok(Self::Mp37),
            "mp38" => Ok(Self::Mp38),
            "mp39" => Ok(Self::Mp39),
            "mp4" => Ok(Self::Mp4),
            "mp40" => Ok(Self::Mp40),
            "mp41" => Ok(Self::Mp41),
            "mp42" => Ok(Self::Mp42),
            "mp43" => Ok(Self::Mp43),
            "mp44" => Ok(Self::Mp44),
            "mp45" => Ok(Self::Mp45),
            "mp46" => Ok(Self::Mp46),
            "mp47" => Ok(Self::Mp47),
            "mp48" => Ok(Self::Mp48),
            "mp49" => Ok(Self::Mp49),
            "mp5" => Ok(Self::Mp5),
            "mp50" => Ok(Self::Mp50),
            "mp51" => Ok(Self::Mp51),
            "mp52" => Ok(Self::Mp52),
            "mp53" => Ok(Self::Mp53),
            "mp54" => Ok(Self::Mp54),
            "mp55" => Ok(Self::Mp55),
            "mp56" => Ok(Self::Mp56),
            "mp57" => Ok(Self::Mp57),
            "mp58" => Ok(Self::Mp58),
            "mp59" => Ok(Self::Mp59),
            "mp6" => Ok(Self::Mp6),
            "mp60" => Ok(Self::Mp60),
            "mp61" => Ok(Self::Mp61),
            "mp62" => Ok(Self::Mp62),
            "mp63" => Ok(Self::Mp63),
            "mp64" => Ok(Self::Mp64),
            "mp65" => Ok(Self::Mp65),
            "mp66" => Ok(Self::Mp66),
            "mp67" => Ok(Self::Mp67),
            "mp68" => Ok(Self::Mp68),
            "mp69" => Ok(Self::Mp69),
            "mp7" => Ok(Self::Mp7),
            "mp70" => Ok(Self::Mp70),
            "mp71" => Ok(Self::Mp71),
            "mp72" => Ok(Self::Mp72),
            "mp73" => Ok(Self::Mp73),
            "mp74" => Ok(Self::Mp74),
            "mp75" => Ok(Self::Mp75),
            "mp76" => Ok(Self::Mp76),
            "mp77" => Ok(Self::Mp77),
            "mp78" => Ok(Self::Mp78),
            "mp79" => Ok(Self::Mp79),
            "mp8" => Ok(Self::Mp8),
            "mp80" => Ok(Self::Mp80),
            "mp81" => Ok(Self::Mp81),
            "mp82" => Ok(Self::Mp82),
            "mp83" => Ok(Self::Mp83),
            "mp84" => Ok(Self::Mp84),
            "mp85" => Ok(Self::Mp85),
            "mp86" => Ok(Self::Mp86),
            "mp87" => Ok(Self::Mp87),
            "mp88" => Ok(Self::Mp88),
            "mp89" => Ok(Self::Mp89),
            "mp9" => Ok(Self::Mp9),
            "mp90" => Ok(Self::Mp90),
            "mp91" => Ok(Self::Mp91),
            "mp92" => Ok(Self::Mp92),
            "mp93" => Ok(Self::Mp93),
            "mp94" => Ok(Self::Mp94),
            "mp95" => Ok(Self::Mp95),
            "mp96" => Ok(Self::Mp96),
            "mp97" => Ok(Self::Mp97),
            "mp98" => Ok(Self::Mp98),
            "mp99" => Ok(Self::Mp99),
            "rootfs" => Ok(Self::Rootfs),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
