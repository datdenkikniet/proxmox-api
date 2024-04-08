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
    pub validation_delay: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
    #[serde(rename = "do")]
    Do,
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
    #[serde(rename = "tele3")]
    Tele3,
    #[serde(rename = "tencent")]
    Tencent,
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
    #[serde(rename = "world4you")]
    World4you,
    #[serde(rename = "yandex")]
    Yandex,
    #[serde(rename = "yc")]
    Yc,
    #[serde(rename = "zilore")]
    Zilore,
    #[serde(rename = "zone")]
    Zone,
    #[serde(rename = "zonomi")]
    Zonomi,
}
