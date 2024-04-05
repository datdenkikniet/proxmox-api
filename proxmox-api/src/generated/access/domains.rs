pub mod realm;
pub struct DomainsClient<T> {
    client: T,
    path: String,
}
impl<T> DomainsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/domains"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Mode {
    #[serde(rename = "ldap")]
    Ldap,
    #[serde(rename = "ldap+starttls")]
    LdapStarttls,
    #[serde(rename = "ldaps")]
    Ldaps,
}
impl Default for Mode {
    fn default() -> Self {
        Self::Ldap
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Sslversion {
    #[serde(rename = "tlsv1")]
    Tlsv1,
    #[serde(rename = "tlsv1_1")]
    Tlsv11,
    #[serde(rename = "tlsv1_2")]
    Tlsv12,
    #[serde(rename = "tlsv1_3")]
    Tlsv13,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Tfa {
    #[serde(rename = "oath")]
    Oath,
    #[serde(rename = "yubico")]
    Yubico,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "ad")]
    Ad,
    #[serde(rename = "ldap")]
    Ldap,
    #[serde(rename = "openid")]
    Openid,
    #[serde(rename = "pam")]
    Pam,
    #[serde(rename = "pve")]
    Pve,
}
impl GetOutputItems {
    pub fn new(realm: String, ty: String) -> Self {
        Self {
            realm,
            ty,
            comment: Default::default(),
            tfa: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A comment. The GUI use this text when you select a domain (Realm) on the login window."]
    pub comment: Option<String>,
    pub realm: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Two-factor authentication provider."]
    pub tfa: Option<Tfa>,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> DomainsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Authentication domain index."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PostParams {
    pub fn new(realm: String, ty: Type) -> Self {
        Self {
            realm,
            ty,
            acr_values: Default::default(),
            autocreate: Default::default(),
            base_dn: Default::default(),
            bind_dn: Default::default(),
            capath: Default::default(),
            case_sensitive: Default::default(),
            cert: Default::default(),
            certkey: Default::default(),
            check_connection: Default::default(),
            client_id: Default::default(),
            client_key: Default::default(),
            comment: Default::default(),
            default: Default::default(),
            domain: Default::default(),
            filter: Default::default(),
            group_classes: Default::default(),
            group_dn: Default::default(),
            group_filter: Default::default(),
            group_name_attr: Default::default(),
            issuer_url: Default::default(),
            mode: Default::default(),
            password: Default::default(),
            port: Default::default(),
            prompt: Default::default(),
            scopes: Default::default(),
            secure: Default::default(),
            server1: Default::default(),
            server2: Default::default(),
            sslversion: Default::default(),
            sync_defaults_options: Default::default(),
            sync_attributes: Default::default(),
            tfa: Default::default(),
            user_attr: Default::default(),
            user_classes: Default::default(),
            username_claim: Default::default(),
            verify: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(rename = "acr-values")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specifies the Authentication Context Class Reference values that theAuthorization Server is being requested to use for the Auth Request."]
    pub acr_values: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Automatically create users if they do not exist."]
    pub autocreate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP base domain name"]
    pub base_dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP bind domain name"]
    pub bind_dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Path to the CA certificate store"]
    pub capath: Option<String>,
    #[serde(rename = "case-sensitive")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "username is case-sensitive"]
    pub case_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Path to the client certificate"]
    pub cert: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Path to the client certificate key"]
    pub certkey: Option<String>,
    #[serde(rename = "check-connection")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Check bind connection to the server."]
    pub check_connection: Option<bool>,
    #[serde(rename = "client-id")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OpenID Client ID"]
    pub client_id: Option<String>,
    #[serde(rename = "client-key")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OpenID Client Key"]
    pub client_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description."]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use this as default realm"]
    pub default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "AD domain name"]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP filter for user sync."]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The objectclasses for groups."]
    pub group_classes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP base domain name for group sync. If not set, the base_dn will be used."]
    pub group_dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP filter for group sync."]
    pub group_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP attribute representing a groups name. If not set or found, the first value of the DN will be used as name."]
    pub group_name_attr: Option<String>,
    #[serde(rename = "issuer-url")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OpenID Issuer Url"]
    pub issuer_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP protocol mode."]
    pub mode: Option<Mode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP bind password. Will be stored in '/etc/pve/priv/realm/\\<REALM\\>.pw'."]
    pub password: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Server port."]
    pub port: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specifies whether the Authorization Server prompts the End-User for reauthentication and consent."]
    pub prompt: Option<String>,
    #[doc = "Authentication domain ID"]
    pub realm: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specifies the scopes (user details) that should be authorized and returned, for example 'email' or 'profile'."]
    pub scopes: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use secure LDAPS protocol. DEPRECATED: use 'mode' instead."]
    pub secure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Server IP address (or DNS name)"]
    pub server1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Fallback Server IP address (or DNS name)"]
    pub server2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAPS TLS/SSL version. It's not recommended to use version older than 1.2!"]
    pub sslversion: Option<Sslversion>,
    #[serde(rename = "sync-defaults-options")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The default options for behavior of synchronizations."]
    pub sync_defaults_options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comma separated list of key=value pairs for specifying which LDAP attributes map to which PVE user field. For example, to map the LDAP attribute 'mail' to PVEs 'email', write  'email=mail'. By default, each PVE user field is represented  by an LDAP attribute of the same name."]
    pub sync_attributes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use Two-factor authentication."]
    pub tfa: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Realm type."]
    pub ty: Type,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP user attribute name"]
    pub user_attr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The objectclasses for users."]
    pub user_classes: Option<String>,
    #[serde(rename = "username-claim")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OpenID claim used to generate the unique username."]
    pub username_claim: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Verify the server's SSL certificate"]
    pub verify: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> DomainsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Add an authentication server."]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> DomainsClient<T>
where
    T: crate::client::Client,
{
    pub fn realm(&self, realm: &str) -> realm::RealmClient<T> {
        realm::RealmClient::<T>::new(self.client.clone(), &self.path, realm)
    }
}
