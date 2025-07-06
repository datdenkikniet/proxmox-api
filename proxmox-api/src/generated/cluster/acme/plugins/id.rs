#[derive(Debug, Clone)]
pub struct IdClient<T> {
    client: T,
    path: String,
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete ACME plugin configuration."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get ACME plugin configuration."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update ACME plugin configuration."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "API plugin name"]
    #[doc = ""]
    pub api: Option<Api>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "DNS plugin data. (base64 encoded)"]
    #[doc = ""]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Flag to disable the config."]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of cluster node names."]
    #[doc = ""]
    pub nodes: Option<String>,
    #[serde(rename = "validation-delay")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Extra delay in seconds to wait before requesting validation. Allows to cope with a long TTL of DNS records."]
    #[doc = ""]
    pub validation_delay: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "API plugin name"]
#[doc = ""]
pub enum Api {
    #[serde(rename = "1984hosting")]
    _1984hosting,
    #[serde(rename = "acmedns")]
    Acmedns,
    #[serde(rename = "acmeproxy")]
    Acmeproxy,
    #[serde(rename = "active24")]
    Active24,
    #[serde(rename = "ad")]
    Ad,
    #[serde(rename = "ali")]
    Ali,
    #[serde(rename = "alviy")]
    Alviy,
    #[serde(rename = "anx")]
    Anx,
    #[serde(rename = "artfiles")]
    Artfiles,
    #[serde(rename = "arvan")]
    Arvan,
    #[serde(rename = "aurora")]
    Aurora,
    #[serde(rename = "autodns")]
    Autodns,
    #[serde(rename = "aws")]
    Aws,
    #[serde(rename = "azion")]
    Azion,
    #[serde(rename = "azure")]
    Azure,
    #[serde(rename = "bookmyname")]
    Bookmyname,
    #[serde(rename = "bunny")]
    Bunny,
    #[serde(rename = "cf")]
    Cf,
    #[serde(rename = "clouddns")]
    Clouddns,
    #[serde(rename = "cloudns")]
    Cloudns,
    #[serde(rename = "cn")]
    Cn,
    #[serde(rename = "conoha")]
    Conoha,
    #[serde(rename = "constellix")]
    Constellix,
    #[serde(rename = "cpanel")]
    Cpanel,
    #[serde(rename = "curanet")]
    Curanet,
    #[serde(rename = "cyon")]
    Cyon,
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "ddnss")]
    Ddnss,
    #[serde(rename = "desec")]
    Desec,
    #[serde(rename = "df")]
    Df,
    #[serde(rename = "dgon")]
    Dgon,
    #[serde(rename = "dnsexit")]
    Dnsexit,
    #[serde(rename = "dnshome")]
    Dnshome,
    #[serde(rename = "dnsimple")]
    Dnsimple,
    #[serde(rename = "dnsservices")]
    Dnsservices,
    #[serde(rename = "doapi")]
    Doapi,
    #[serde(rename = "domeneshop")]
    Domeneshop,
    #[serde(rename = "dp")]
    Dp,
    #[serde(rename = "dpi")]
    Dpi,
    #[serde(rename = "dreamhost")]
    Dreamhost,
    #[serde(rename = "duckdns")]
    Duckdns,
    #[serde(rename = "durabledns")]
    Durabledns,
    #[serde(rename = "dyn")]
    Dyn,
    #[serde(rename = "dynu")]
    Dynu,
    #[serde(rename = "dynv6")]
    Dynv6,
    #[serde(rename = "easydns")]
    Easydns,
    #[serde(rename = "edgedns")]
    Edgedns,
    #[serde(rename = "euserv")]
    Euserv,
    #[serde(rename = "exoscale")]
    Exoscale,
    #[serde(rename = "fornex")]
    Fornex,
    #[serde(rename = "freedns")]
    Freedns,
    #[serde(rename = "gandi_livedns")]
    GandiLivedns,
    #[serde(rename = "gcloud")]
    Gcloud,
    #[serde(rename = "gcore")]
    Gcore,
    #[serde(rename = "gd")]
    Gd,
    #[serde(rename = "geoscaling")]
    Geoscaling,
    #[serde(rename = "googledomains")]
    Googledomains,
    #[serde(rename = "he")]
    He,
    #[serde(rename = "hetzner")]
    Hetzner,
    #[serde(rename = "hexonet")]
    Hexonet,
    #[serde(rename = "hostingde")]
    Hostingde,
    #[serde(rename = "huaweicloud")]
    Huaweicloud,
    #[serde(rename = "infoblox")]
    Infoblox,
    #[serde(rename = "infomaniak")]
    Infomaniak,
    #[serde(rename = "internetbs")]
    Internetbs,
    #[serde(rename = "inwx")]
    Inwx,
    #[serde(rename = "ionos")]
    Ionos,
    #[serde(rename = "ionos_cloud")]
    IonosCloud,
    #[serde(rename = "ipv64")]
    Ipv64,
    #[serde(rename = "ispconfig")]
    Ispconfig,
    #[serde(rename = "jd")]
    Jd,
    #[serde(rename = "joker")]
    Joker,
    #[serde(rename = "kappernet")]
    Kappernet,
    #[serde(rename = "kas")]
    Kas,
    #[serde(rename = "kinghost")]
    Kinghost,
    #[serde(rename = "knot")]
    Knot,
    #[serde(rename = "la")]
    La,
    #[serde(rename = "leaseweb")]
    Leaseweb,
    #[serde(rename = "lexicon")]
    Lexicon,
    #[serde(rename = "limacity")]
    Limacity,
    #[serde(rename = "linode")]
    Linode,
    #[serde(rename = "linode_v4")]
    LinodeV4,
    #[serde(rename = "loopia")]
    Loopia,
    #[serde(rename = "lua")]
    Lua,
    #[serde(rename = "maradns")]
    Maradns,
    #[serde(rename = "me")]
    Me,
    #[serde(rename = "miab")]
    Miab,
    #[serde(rename = "misaka")]
    Misaka,
    #[serde(rename = "myapi")]
    Myapi,
    #[serde(rename = "mydevil")]
    Mydevil,
    #[serde(rename = "mydnsjp")]
    Mydnsjp,
    #[serde(rename = "mythic_beasts")]
    MythicBeasts,
    #[serde(rename = "namecheap")]
    Namecheap,
    #[serde(rename = "namecom")]
    Namecom,
    #[serde(rename = "namesilo")]
    Namesilo,
    #[serde(rename = "nanelo")]
    Nanelo,
    #[serde(rename = "nederhost")]
    Nederhost,
    #[serde(rename = "neodigit")]
    Neodigit,
    #[serde(rename = "netcup")]
    Netcup,
    #[serde(rename = "netlify")]
    Netlify,
    #[serde(rename = "nic")]
    Nic,
    #[serde(rename = "njalla")]
    Njalla,
    #[serde(rename = "nm")]
    Nm,
    #[serde(rename = "nsd")]
    Nsd,
    #[serde(rename = "nsone")]
    Nsone,
    #[serde(rename = "nsupdate")]
    Nsupdate,
    #[serde(rename = "nw")]
    Nw,
    #[serde(rename = "oci")]
    Oci,
    #[serde(rename = "omglol")]
    Omglol,
    #[serde(rename = "one")]
    One,
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "openprovider")]
    Openprovider,
    #[serde(rename = "openstack")]
    Openstack,
    #[serde(rename = "opnsense")]
    Opnsense,
    #[serde(rename = "ovh")]
    Ovh,
    #[serde(rename = "pdns")]
    Pdns,
    #[serde(rename = "pleskxml")]
    Pleskxml,
    #[serde(rename = "pointhq")]
    Pointhq,
    #[serde(rename = "porkbun")]
    Porkbun,
    #[serde(rename = "rackcorp")]
    Rackcorp,
    #[serde(rename = "rackspace")]
    Rackspace,
    #[serde(rename = "rage4")]
    Rage4,
    #[serde(rename = "rcode0")]
    Rcode0,
    #[serde(rename = "regru")]
    Regru,
    #[serde(rename = "scaleway")]
    Scaleway,
    #[serde(rename = "schlundtech")]
    Schlundtech,
    #[serde(rename = "selectel")]
    Selectel,
    #[serde(rename = "selfhost")]
    Selfhost,
    #[serde(rename = "servercow")]
    Servercow,
    #[serde(rename = "simply")]
    Simply,
    #[serde(rename = "technitium")]
    Technitium,
    #[serde(rename = "tele3")]
    Tele3,
    #[serde(rename = "tencent")]
    Tencent,
    #[serde(rename = "timeweb")]
    Timeweb,
    #[serde(rename = "transip")]
    Transip,
    #[serde(rename = "udr")]
    Udr,
    #[serde(rename = "ultra")]
    Ultra,
    #[serde(rename = "unoeuro")]
    Unoeuro,
    #[serde(rename = "variomedia")]
    Variomedia,
    #[serde(rename = "veesp")]
    Veesp,
    #[serde(rename = "vercel")]
    Vercel,
    #[serde(rename = "vscale")]
    Vscale,
    #[serde(rename = "vultr")]
    Vultr,
    #[serde(rename = "websupport")]
    Websupport,
    #[serde(rename = "west_cn")]
    WestCn,
    #[serde(rename = "world4you")]
    World4you,
    #[serde(rename = "yandex360")]
    Yandex360,
    #[serde(rename = "yc")]
    Yc,
    #[serde(rename = "zilore")]
    Zilore,
    #[serde(rename = "zone")]
    Zone,
    #[serde(rename = "zoneedit")]
    Zoneedit,
    #[serde(rename = "zonomi")]
    Zonomi,
}
impl TryFrom<&str> for Api {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "1984hosting" => Ok(Self::_1984hosting),
            "acmedns" => Ok(Self::Acmedns),
            "acmeproxy" => Ok(Self::Acmeproxy),
            "active24" => Ok(Self::Active24),
            "ad" => Ok(Self::Ad),
            "ali" => Ok(Self::Ali),
            "alviy" => Ok(Self::Alviy),
            "anx" => Ok(Self::Anx),
            "artfiles" => Ok(Self::Artfiles),
            "arvan" => Ok(Self::Arvan),
            "aurora" => Ok(Self::Aurora),
            "autodns" => Ok(Self::Autodns),
            "aws" => Ok(Self::Aws),
            "azion" => Ok(Self::Azion),
            "azure" => Ok(Self::Azure),
            "bookmyname" => Ok(Self::Bookmyname),
            "bunny" => Ok(Self::Bunny),
            "cf" => Ok(Self::Cf),
            "clouddns" => Ok(Self::Clouddns),
            "cloudns" => Ok(Self::Cloudns),
            "cn" => Ok(Self::Cn),
            "conoha" => Ok(Self::Conoha),
            "constellix" => Ok(Self::Constellix),
            "cpanel" => Ok(Self::Cpanel),
            "curanet" => Ok(Self::Curanet),
            "cyon" => Ok(Self::Cyon),
            "da" => Ok(Self::Da),
            "ddnss" => Ok(Self::Ddnss),
            "desec" => Ok(Self::Desec),
            "df" => Ok(Self::Df),
            "dgon" => Ok(Self::Dgon),
            "dnsexit" => Ok(Self::Dnsexit),
            "dnshome" => Ok(Self::Dnshome),
            "dnsimple" => Ok(Self::Dnsimple),
            "dnsservices" => Ok(Self::Dnsservices),
            "doapi" => Ok(Self::Doapi),
            "domeneshop" => Ok(Self::Domeneshop),
            "dp" => Ok(Self::Dp),
            "dpi" => Ok(Self::Dpi),
            "dreamhost" => Ok(Self::Dreamhost),
            "duckdns" => Ok(Self::Duckdns),
            "durabledns" => Ok(Self::Durabledns),
            "dyn" => Ok(Self::Dyn),
            "dynu" => Ok(Self::Dynu),
            "dynv6" => Ok(Self::Dynv6),
            "easydns" => Ok(Self::Easydns),
            "edgedns" => Ok(Self::Edgedns),
            "euserv" => Ok(Self::Euserv),
            "exoscale" => Ok(Self::Exoscale),
            "fornex" => Ok(Self::Fornex),
            "freedns" => Ok(Self::Freedns),
            "gandi_livedns" => Ok(Self::GandiLivedns),
            "gcloud" => Ok(Self::Gcloud),
            "gcore" => Ok(Self::Gcore),
            "gd" => Ok(Self::Gd),
            "geoscaling" => Ok(Self::Geoscaling),
            "googledomains" => Ok(Self::Googledomains),
            "he" => Ok(Self::He),
            "hetzner" => Ok(Self::Hetzner),
            "hexonet" => Ok(Self::Hexonet),
            "hostingde" => Ok(Self::Hostingde),
            "huaweicloud" => Ok(Self::Huaweicloud),
            "infoblox" => Ok(Self::Infoblox),
            "infomaniak" => Ok(Self::Infomaniak),
            "internetbs" => Ok(Self::Internetbs),
            "inwx" => Ok(Self::Inwx),
            "ionos" => Ok(Self::Ionos),
            "ionos_cloud" => Ok(Self::IonosCloud),
            "ipv64" => Ok(Self::Ipv64),
            "ispconfig" => Ok(Self::Ispconfig),
            "jd" => Ok(Self::Jd),
            "joker" => Ok(Self::Joker),
            "kappernet" => Ok(Self::Kappernet),
            "kas" => Ok(Self::Kas),
            "kinghost" => Ok(Self::Kinghost),
            "knot" => Ok(Self::Knot),
            "la" => Ok(Self::La),
            "leaseweb" => Ok(Self::Leaseweb),
            "lexicon" => Ok(Self::Lexicon),
            "limacity" => Ok(Self::Limacity),
            "linode" => Ok(Self::Linode),
            "linode_v4" => Ok(Self::LinodeV4),
            "loopia" => Ok(Self::Loopia),
            "lua" => Ok(Self::Lua),
            "maradns" => Ok(Self::Maradns),
            "me" => Ok(Self::Me),
            "miab" => Ok(Self::Miab),
            "misaka" => Ok(Self::Misaka),
            "myapi" => Ok(Self::Myapi),
            "mydevil" => Ok(Self::Mydevil),
            "mydnsjp" => Ok(Self::Mydnsjp),
            "mythic_beasts" => Ok(Self::MythicBeasts),
            "namecheap" => Ok(Self::Namecheap),
            "namecom" => Ok(Self::Namecom),
            "namesilo" => Ok(Self::Namesilo),
            "nanelo" => Ok(Self::Nanelo),
            "nederhost" => Ok(Self::Nederhost),
            "neodigit" => Ok(Self::Neodigit),
            "netcup" => Ok(Self::Netcup),
            "netlify" => Ok(Self::Netlify),
            "nic" => Ok(Self::Nic),
            "njalla" => Ok(Self::Njalla),
            "nm" => Ok(Self::Nm),
            "nsd" => Ok(Self::Nsd),
            "nsone" => Ok(Self::Nsone),
            "nsupdate" => Ok(Self::Nsupdate),
            "nw" => Ok(Self::Nw),
            "oci" => Ok(Self::Oci),
            "omglol" => Ok(Self::Omglol),
            "one" => Ok(Self::One),
            "online" => Ok(Self::Online),
            "openprovider" => Ok(Self::Openprovider),
            "openstack" => Ok(Self::Openstack),
            "opnsense" => Ok(Self::Opnsense),
            "ovh" => Ok(Self::Ovh),
            "pdns" => Ok(Self::Pdns),
            "pleskxml" => Ok(Self::Pleskxml),
            "pointhq" => Ok(Self::Pointhq),
            "porkbun" => Ok(Self::Porkbun),
            "rackcorp" => Ok(Self::Rackcorp),
            "rackspace" => Ok(Self::Rackspace),
            "rage4" => Ok(Self::Rage4),
            "rcode0" => Ok(Self::Rcode0),
            "regru" => Ok(Self::Regru),
            "scaleway" => Ok(Self::Scaleway),
            "schlundtech" => Ok(Self::Schlundtech),
            "selectel" => Ok(Self::Selectel),
            "selfhost" => Ok(Self::Selfhost),
            "servercow" => Ok(Self::Servercow),
            "simply" => Ok(Self::Simply),
            "technitium" => Ok(Self::Technitium),
            "tele3" => Ok(Self::Tele3),
            "tencent" => Ok(Self::Tencent),
            "timeweb" => Ok(Self::Timeweb),
            "transip" => Ok(Self::Transip),
            "udr" => Ok(Self::Udr),
            "ultra" => Ok(Self::Ultra),
            "unoeuro" => Ok(Self::Unoeuro),
            "variomedia" => Ok(Self::Variomedia),
            "veesp" => Ok(Self::Veesp),
            "vercel" => Ok(Self::Vercel),
            "vscale" => Ok(Self::Vscale),
            "vultr" => Ok(Self::Vultr),
            "websupport" => Ok(Self::Websupport),
            "west_cn" => Ok(Self::WestCn),
            "world4you" => Ok(Self::World4you),
            "yandex360" => Ok(Self::Yandex360),
            "yc" => Ok(Self::Yc),
            "zilore" => Ok(Self::Zilore),
            "zone" => Ok(Self::Zone),
            "zoneedit" => Ok(Self::Zoneedit),
            "zonomi" => Ok(Self::Zonomi),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
