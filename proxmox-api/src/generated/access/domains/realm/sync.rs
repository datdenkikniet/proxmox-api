pub struct SyncClient<T> {
    client: T,
    path: String,
}
impl<T> SyncClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/sync"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a SyncClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> SyncClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Syncs users and/or groups from the configured LDAP to user.cfg. NOTE: Synced groups will have the name 'name-$realm', so make sure those groups do not exist to prevent overwriting."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, String, T::Error> for &SyncClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<String, T::Error> {
        self.post(params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(rename = "dry-run")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If set, does not write anything."]
    #[doc = ""]
    pub dry_run: Option<bool>,
    #[serde(rename = "enable-new")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable newly synced users immediately."]
    #[doc = ""]
    pub enable_new: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "DEPRECATED: use 'remove-vanished' instead. If set, uses the LDAP Directory as source of truth, deleting users or groups not returned from the sync and removing all locally modified properties of synced users. If not set, only syncs information which is present in the synced data, and does not delete or modify anything else."]
    #[doc = ""]
    pub full: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "DEPRECATED: use 'remove-vanished' instead. Remove ACLs for users or groups which were removed from the config during a sync."]
    #[doc = ""]
    pub purge: Option<bool>,
    #[serde(rename = "remove-vanished")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A semicolon-seperated list of things to remove when they or the user vanishes during a sync. The following values are possible: 'entry' removes the user/group when not returned from the sync. 'properties' removes the set properties on existing user/group that do not appear in the source (even custom ones). 'acl' removes acls when the user/group is not returned from the sync. Instead of a list it also can be 'none' (the default)."]
    #[doc = ""]
    pub remove_vanished: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Select what to sync."]
    #[doc = ""]
    pub scope: Option<Scope>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Select what to sync."]
#[doc = ""]
pub enum Scope {
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "groups")]
    Groups,
    #[serde(rename = "users")]
    Users,
}
impl TryFrom<&str> for Scope {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "both" => Ok(Self::Both),
            "groups" => Ok(Self::Groups),
            "users" => Ok(Self::Users),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
