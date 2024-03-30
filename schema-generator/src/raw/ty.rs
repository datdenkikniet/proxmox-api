use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Type<'a> {
    #[serde(flatten, borrow, default, skip_serializing_if = "Option::is_none")]
    pub ty: Option<TypeKind<'a>>,

    #[serde(default)]
    pub optional: Option<serde_json::Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum TypeKind<'a> {
    Null,
    String,
    Number,
    Integer,
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
