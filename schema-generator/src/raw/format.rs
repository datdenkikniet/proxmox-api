use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use super::Type;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Format<'a> {
    Kind(KnownFormat),
    #[serde(borrow)]
    Properties(HashMap<Cow<'a, str>, Type<'a>>),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum KnownFormat {
    Ipv4,
    #[serde(rename = "ipv4mask")]
    Ipv4Mask,
    #[serde(rename = "CIDRv4")]
    CidrV4,
    Ipv6,
    #[serde(rename = "CIDRv6")]
    CidrV6,
    EmailOpt,
    EmailList,
    #[serde(rename = "IPorCIDRorAlias")]
    IpOrCidrOrAlias,
    #[serde(rename = "IPorCIDR")]
    IpOrCidr,
    InternetAddress,
    #[serde(alias = "CIDR")]
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
    IpList,
    LdapSimpleAttr,
    LdapSimpleAttrList,
    GraphitePath,
    #[serde(rename = "urlencoded")]
    UrlEncoded,

    LxcIpWithLlIfaceList,
    PemCertificate,
    PemCertificateChain,
    PemString,
    MacAddr,
    MacPrefix,
    RealmSyncOptions,
    DiskSize,

    // PVE specific
    PveAcmeDomain,
    PveAcmeDomainList,
    PveAcmeAlias,
    PveLxcMpString,
    PveVolumeIdOrQmPath,
    PveVolumeIdOrAbsolutePath,
    PveDayOfWeekList,
    PveTfaConfig,
    PveDirOverrideList,
    PveCtTimezone,
    PvePrivList,
    PveRealm,
    PveReplicationJobId,
    ProxmoxRemote,
    #[serde(alias = "pve-bridgeid")]
    PveBridgeId,
    PveBridgeIdList,
    PveIface,
    PveIfaceList,
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
    #[serde(rename = "pve-roleid-list")]
    PveRoleIdList,
    #[serde(rename = "pve-tokenid")]
    PveTokenId,
    #[serde(rename = "pve-tokenid-list")]
    PveTokenIdList,
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
    PveQmUsbDevice,

    PveFwConntrackHelper,
    PveFwSportSpec,
    PveFwDportSpec,
    PveFwAddrSpec,
    PveFwProtocolSpec,
    PveFwIcmpTypeSpec,

    PveIpv4Config,
    PveIpv6Config,
}
