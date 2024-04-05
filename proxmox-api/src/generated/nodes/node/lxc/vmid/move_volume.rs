pub struct MoveVolumeClient<T> {
    client: T,
    path: String,
}
impl<T> MoveVolumeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/move_volume"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum TargetVolume {
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
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Volume {
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
}
impl PostParams {
    pub fn new(volume: Volume) -> Self {
        Self {
            volume,
            bwlimit: Default::default(),
            delete: Default::default(),
            digest: Default::default(),
            storage: Default::default(),
            target_digest: Default::default(),
            target_vmid: Default::default(),
            target_volume: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    pub bwlimit: Option<()>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Delete the original volume after successful copy. By default the original is kept as an unused volume entry."]
    pub delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has different SHA1 \" .\n\t\t    \"digest. This can be used to prevent concurrent modifications."]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target Storage."]
    pub storage: Option<String>,
    #[serde(rename = "target-digest")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file of the target \" .\n\t\t    \"container has a different SHA1 digest. This can be used to prevent \" .\n\t\t    \"concurrent modifications."]
    pub target_digest: Option<String>,
    #[serde(rename = "target-vmid")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The (unique) ID of the VM."]
    pub target_vmid: Option<crate::VmId>,
    #[serde(rename = "target-volume")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The config key the volume will be moved to. Default is the source volume key."]
    pub target_volume: Option<TargetVolume>,
    #[doc = "Volume which will be moved."]
    pub volume: Volume,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> MoveVolumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Move a rootfs-/mp-volume to a different storage or to a different container."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
