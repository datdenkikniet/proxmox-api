use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
};

use serde::{Deserialize, Serialize};

use super::Optional;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Type<'a> {
    #[serde(flatten, borrow, default, skip_serializing_if = "Option::is_none")]
    pub ty: Option<TypeKind<'a>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Optional::is_empty")]
    pub optional: Optional,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verbose_description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ty_text: Option<Cow<'a, str>>,
    // format
    // pattern
    // Another field in the parent object is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renderer: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum TypeKind<'a> {
    Null,
    String {
        #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
        max_length: Option<u32>,
        #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
        min_length: Option<u32>,
    },
    Number {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum: Option<u32>,
    },
    Integer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum: Option<u32>,
    },
    Boolean,
    Array {
        items: Box<Type<'a>>,
    },
    Object {
        #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
        properties: Option<BTreeMap<Cow<'a, str>, Type<'a>>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        additional_properties: Option<serde_json::Value>,
    },
    Enum {
        #[serde(rename = "enum")]
        values: HashSet<Cow<'a, str>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<Cow<'a, str>>,
    },

    IpOrCidrOrAlias,
    InternetAddress,
    Cidr,
    ArgumentList,
    StoragePairList,
    DnsName,
    BridgePairList,
    EmailOrUsernameList,
    BackupPerformance,
    PruneBackups,
    StringList,

    // PVE specific
    PvePrivList,
    PveRealm,
    PveReplicationJobId,
    ProxmoxRemote,
    PveBridgeIdList,
    PveIface,
    PveTaskStatusTypeList,
    PveCommandBatch,
    PveNode,
    PveVmId,
    PveVmidList,
    PveHaResourceOrVmId,
    PveCalendarEvent,

    PvePoolId,
    PveUserId,
    PveGroupId,
    PveRoleId,

    PveConfigId,
    PveConfigIdList,

    PveStoragePortalDns,
    PveStorageServer,
    PveStorageId,
    PveStorageIdList,
    PveStorageContent,
    PveStorageContentList,
}
