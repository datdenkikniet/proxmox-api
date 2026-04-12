use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
    hash::Hash,
};

use serde::{Deserialize, Serialize};

use super::Type;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Format<'a> {
    Kind(KnownFormat),
    #[serde(borrow)]
    Properties(BTreeMap<Cow<'a, str>, Type<'a>>),
}

macro_rules ! known_format {
    ($(
        $(#[serde($($serde_body:tt)*)])?
        $name:ident,
    )*) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[serde(rename_all = "kebab-case")]
        pub enum KnownFormat {
            $(
                $(#[serde($($serde_body)*)])?
                $name,
            )*
            MacAddr(#[serde(skip)] bool),
            #[serde(untagged)]
            Unknown(String),
        }

        impl std::fmt::Display for KnownFormat {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$name => f.write_str(stringify!($name)),
                    )*
                    Self::MacAddr(false) => f.write_str("MacAddr(false)"),
                    Self::MacAddr(true) => f.write_str("MacAddr(true)"),
                    Self::Unknown(_) => f.write_str("Unknown"),
                }
            }
        }

        pub static ALL_KNOWN_FORMATS: std::sync::LazyLock<HashSet<KnownFormat>> = std::sync::LazyLock::new(|| HashSet::from([
            $(
                KnownFormat::$name,
            )*
            KnownFormat::MacAddr(false),
            KnownFormat::MacAddr(true),
        ]));
    }
}

known_format!(
    String,
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
    EmailOrUsername,
    EmailOrUsernameList,
    BackupPerformance,
    BackupFleecing,
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
    PveSdnBgpRtList,
    PveSdnControllerId,
    PveSdnDhcpRange,
    PveSdnDnsId,
    PveSdnIpamId,
    PveSdnIsisNet,
    PveSdnSubnetId,
    PveSdnVnetId,
    PveSdnZoneId,
    PveFwConntrackHelper,
    PveFwSportSpec,
    PveFwDportSpec,
    PveFwAddrSpec,
    PveFwProtocolSpec,
    PveFwIcmpTypeSpec,
    PveIpv4Config,
    PveIpv6Config,
);
