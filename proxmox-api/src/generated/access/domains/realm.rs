pub mod sync;
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
impl<T> RealmClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete an authentication server."]
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
    pub fn get(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
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
    #[doc = "A list of settings you want to delete."]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    pub digest: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LDAP user attribute name"]
    pub user_attr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The objectclasses for users."]
    pub user_classes: Option<String>,
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
impl<T> RealmClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update authentication server settings."]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
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
