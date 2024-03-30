use serde::{Deserialize, Serialize};

use super::Type;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Format<'a> {
    Kind(KnownFormat),
    #[serde(borrow)]
    Properties(Type<'a>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum KnownFormat {
    EmailOpt,
    #[serde(rename = "IPorCIDRorAlias")]
    IpOrCidrOrAlias,
    InternetAddress,
    Cidr,
    #[serde(rename = "string-alist")]
    ArgumentList,
    StoragePairList,
    DnsName,
    DnsNameList,
    BridgePairList,
    EmailOrUsernameList,
    BackupPerformance,
    PruneBackups,
    StringList,
    Address,
    AddressList,
    Ip,
    LdapSimpleAttr,
    LdapSimpleAttrList,
    GraphitePath,
    #[serde(rename = "urlencoded")]
    UrlEncoded,

    LxcIpWithLlIfaceList,
    PemCertificate,
    MacAddr,
    RealmSyncOptions,

    // PVE specific
    PveTfaConfig,
    PveDirOverrideList,
    PveCtTimezone,
    PvePrivList,
    PveRealm,
    PveReplicationJobId,
    ProxmoxRemote,
    PveBridgeIdList,
    PveIface,
    PveTaskStatusTypeList,
    PveCommandBatch,
    PveNode,
    PveNodeList,
    #[serde(rename = "pve-vmid")]
    PveVmId,
    #[serde(rename = "pve-vmid-list")]
    PveVmidList,
    PveVmCpuConf,
    #[serde(rename = "pve-ha-resource-or-vm-id")]
    PveHaResourceOrVmId,
    PveCalendarEvent,
    PveHaGroupNodeList,
    PveVolumeId,
    PveHotplugFeatures,
    PveStartupOrder,
    PveTagList,

    #[serde(rename = "pve-poolid")]
    PvePoolId,
    #[serde(rename = "pve-userid")]
    PveUserId,
    #[serde(rename = "pve-userid-list")]
    PveUserIdList,
    #[serde(rename = "pve-groupid")]
    PveGroupId,
    #[serde(rename = "pve-groupid-list")]
    PveGroupIdList,
    #[serde(rename = "pve-roleid")]
    PveRoleId,

    #[serde(rename = "pve-configid")]
    PveConfigId,
    #[serde(rename = "pve-configid-list")]
    PveConfigIdList,

    PveStoragePortalDns,
    PveStorageServer,
    PveStorageId,
    PveStorageIdList,
    PveStorageContent,
    PveStorageContentList,
    PveStoragePath,
    PveStorageFormat,
    PveStoragePortalDnsList,
    PveStorageOptions,
    #[serde(rename = "pve-storage-vgname")]
    PveStorageVgName,

    #[serde(rename = "pve-cpuset")]
    PveCpuSet,

    PveQmBoot,
    #[serde(rename = "pve-qm-bootdisk")]
    PveQmBootDisk,
    PveQmIde,
    #[serde(rename = "pve-qm-cicustom")]
    PveQmCiCustom,
    PveQmHostpci,
    PveQmWatchdog,
    #[serde(rename = "pve-qm-ipconfig")]
    PveQmIpConfig,
    PveQmSmbios1,

    PveFwConntrackHelper,
}
