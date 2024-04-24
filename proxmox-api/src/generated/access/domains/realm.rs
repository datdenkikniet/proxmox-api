pub mod sync;
#[derive(Debug, Clone)]
pub struct RealmClient<T> {
    client: T,
    path: String,
}
impl<T> RealmClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, realm: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, realm),
        }
    }
}
impl<T> RealmClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete an authentication server."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> RealmClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get auth server configuration."]
    #[doc = ""]
    pub fn get(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> RealmClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update authentication server settings."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(rename = "acr-values")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specifies the Authentication Context Class Reference values that theAuthorization Server is being requested to use for the Auth Request."]
    #[doc = ""]
    pub acr_values: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Automatically create users if they do not exist."]
    #[doc = ""]
    pub autocreate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP base domain name"]
    #[doc = ""]
    pub base_dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP bind domain name"]
    #[doc = ""]
    pub bind_dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Path to the CA certificate store"]
    #[doc = ""]
    pub capath: Option<String>,
    #[serde(rename = "case-sensitive")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "username is case-sensitive"]
    #[doc = ""]
    pub case_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Path to the client certificate"]
    #[doc = ""]
    pub cert: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Path to the client certificate key"]
    #[doc = ""]
    pub certkey: Option<String>,
    #[serde(rename = "check-connection")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Check bind connection to the server."]
    #[doc = ""]
    pub check_connection: Option<bool>,
    #[serde(rename = "client-id")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OpenID Client ID"]
    #[doc = ""]
    pub client_id: Option<String>,
    #[serde(rename = "client-key")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OpenID Client Key"]
    #[doc = ""]
    pub client_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description."]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use this as default realm"]
    #[doc = ""]
    pub default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "AD domain name"]
    #[doc = ""]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP filter for user sync."]
    #[doc = ""]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The objectclasses for groups."]
    #[doc = ""]
    pub group_classes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP base domain name for group sync. If not set, the base_dn will be used."]
    #[doc = ""]
    pub group_dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP filter for group sync."]
    #[doc = ""]
    pub group_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP attribute representing a groups name. If not set or found, the first value of the DN will be used as name."]
    #[doc = ""]
    pub group_name_attr: Option<String>,
    #[serde(rename = "issuer-url")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OpenID Issuer Url"]
    #[doc = ""]
    pub issuer_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP protocol mode."]
    #[doc = ""]
    pub mode: Option<Mode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP bind password. Will be stored in '/etc/pve/priv/realm/\\\\<REALM\\\\>.pw'."]
    #[doc = ""]
    pub password: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Server port."]
    #[doc = ""]
    pub port: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specifies whether the Authorization Server prompts the End-User for reauthentication and consent."]
    #[doc = ""]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specifies the scopes (user details) that should be authorized and returned, for example 'email' or 'profile'."]
    #[doc = ""]
    pub scopes: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use secure LDAPS protocol. DEPRECATED: use 'mode' instead."]
    #[doc = ""]
    pub secure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Server IP address (or DNS name)"]
    #[doc = ""]
    pub server1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Fallback Server IP address (or DNS name)"]
    #[doc = ""]
    pub server2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAPS TLS/SSL version. It's not recommended to use version older than 1.2!"]
    #[doc = ""]
    pub sslversion: Option<Sslversion>,
    #[serde(rename = "sync-defaults-options")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The default options for behavior of synchronizations."]
    #[doc = ""]
    pub sync_defaults_options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comma separated list of key=value pairs for specifying which LDAP attributes map to which PVE user field. For example, to map the LDAP attribute 'mail' to PVEs 'email', write  'email=mail'. By default, each PVE user field is represented  by an LDAP attribute of the same name."]
    #[doc = ""]
    pub sync_attributes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use Two-factor authentication."]
    #[doc = ""]
    pub tfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP user attribute name"]
    #[doc = ""]
    pub user_attr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The objectclasses for users."]
    #[doc = ""]
    pub user_classes: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Verify the server's SSL certificate"]
    #[doc = ""]
    pub verify: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "LDAP protocol mode."]
#[doc = ""]
pub enum Mode {
    #[serde(rename = "ldap")]
    Ldap,
    #[serde(rename = "ldap+starttls")]
    LdapStarttls,
    #[serde(rename = "ldaps")]
    Ldaps,
}
impl TryFrom<&str> for Mode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ldap" => Ok(Self::Ldap),
            "ldap+starttls" => Ok(Self::LdapStarttls),
            "ldaps" => Ok(Self::Ldaps),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Mode {
    fn default() -> Self {
        Self::Ldap
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "LDAPS TLS/SSL version. It's not recommended to use version older than 1.2!"]
#[doc = ""]
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
impl TryFrom<&str> for Sslversion {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "tlsv1" => Ok(Self::Tlsv1),
            "tlsv1_1" => Ok(Self::Tlsv11),
            "tlsv1_2" => Ok(Self::Tlsv12),
            "tlsv1_3" => Ok(Self::Tlsv13),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> RealmClient<T>
where
    T: crate::client::Client,
{
    pub fn sync(&self) -> sync::SyncClient<T> {
        sync::SyncClient::<T>::new(self.client.clone(), &self.path)
    }
}
