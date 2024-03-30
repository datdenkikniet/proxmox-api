use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Type<'a> {
    #[serde(default)]
    pub optional: Option<serde_json::Value>,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    #[serde(flatten, borrow)]
    pub ty: TypeKind<'a>,
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
        #[serde(borrow)]
        properties: BTreeMap<Cow<'a, str>, Type<'a>>,
        additional_properties: Option<serde_json::Value>,
    },
    Enum {
        #[serde(rename = "enum")]
        values: HashSet<Cow<'a, str>>,
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
