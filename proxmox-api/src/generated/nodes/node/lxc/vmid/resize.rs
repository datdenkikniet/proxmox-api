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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
    pub digest: Option<String>,
    #[doc = "The disk you want to resize."]
    pub disk: Disk,
    #[doc = "The new size. With the '+' sign the value is added to the actual size of the volume and without it, the value is taken as an absolute one. Shrinking disk size is not supported."]
    pub size: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ResizeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Resize a container mount point."]
    pub fn put(&self, params: PutParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
