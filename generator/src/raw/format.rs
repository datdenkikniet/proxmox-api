use std::{borrow::Cow, collections::BTreeMap};

use serde::{Deserialize, Serialize};

use super::Type;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Format<'a> {
    #[serde(borrow)]
    Properties(BTreeMap<Cow<'a, str>, Type<'a>>),
    Kind(KnownFormat),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(from = "&str")]
pub struct KnownFormat {
    pub is_list: bool,
    pub ty: KnownFormatType,
}

impl From<&str> for KnownFormat {
    fn from(value: &str) -> Self {
        let (value, is_list) = if value.ends_with("-list") {
            (&value[..value.len() - 5], true)
        } else {
            (value, false)
        };

        let ty: KnownFormatType = TryFrom::try_from(value)
            .map_err(|_| format!("Unknown format {value}"))
            .unwrap();

        KnownFormat { is_list, ty }
    }
}

impl Into<String> for KnownFormat {
    fn into(self) -> String {
        todo!()
    }
}

macro_rules ! known_format_type {
    ($([$name:ident, $string_repr:literal],)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Serialize)]
        pub enum KnownFormatType {
            $($name,)*
            MacAddr(bool),
        }

        impl TryFrom<&str> for KnownFormatType {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                let value = match value {
                    $(
                        $string_repr => Self::$name,
                    )*
                    "mac-addr" => Self::MacAddr(false),
                    _ => return Err(()),
                };

                Ok(value)
            }
        }
    }
}

#[rustfmt::skip]
known_format_type!(
    [String, "string"],

    [Ip, "ip"],
    [Cidr, "CIDR"],
    [IpOrCidr, "IPorCIDR"],
    [Address, "address"],
    [IpOrCidrOrAlias, "IPorCIDRorAlias"],
    [InternetAddress, "internet-address"],
    [DnsName, "dns-name"],

    [Ipv4, "ipv4"],
    [Ipv4Mask, "ipv4mask"],
    [CidrV4, "CIDRv4"],

    [Ipv6, "ipv6"],
    [Ipv6Mask, "ipv6mask"],
    [CidrV6, "CIDRv6"],

    [EmailOpt, "email-opt"],
    [Email, "email"],
    [EmailOrUsername, "email-or-username"],

    [ArgumentList, "string-alist"],

    [StoragePair, "storage-pair"],
    [BridgePair, "bridge-pair"],

    [BackupPerformance, "backup-performance"],
    [PruneBackups, "prune-backups"],

    [LdapSimpleAttr, "ldap-simple-attr"],
    [GraphitePath, "graphite-path"],
    [UrlEncoded, "urlencoded"],

    [LxcIpWithLlIface, "lxc-ip-with-ll-iface"],
    [MacPrefix, "mac-prefix"],

    [PemCertificate, "pem-certificate"],
    [PemCertificateChain, "pem-certificate-chain"],
    [PemString, "pem-string"],

    [RealmSyncOptions, "realm-sync-options"],
    [DiskSize, "disk-size"],

    [ProxmoxRemote, "proxmox-remote"],

    // PVE specific
    [PveAcmeDomain, "pve-acme-domain"],
    [PveAcmeAlias, "pve-acme-alias"],
    [PveLxcMpString, "pve-lxc-mp-string"],
    [PveVolumeIdOrQmPath, "pve-volume-id-or-qm-path"],
    [PveVolumeIdOrAbsolutePath, "pve-volume-id-or-absolute-path"],
    [PveDayOfWeek, "pve-day-of-week"],
    [PveTfaConfig, "pve-tfa-config"],
    [PveDirOverride, "pve-dir-override"],
    [PveCtTimezone, "pve-ct-timezone"],
    [PvePriv, "pve-priv"],
    [PveRealm, "pve-realm"],
    [PveReplicationJobId, "pve-replication-job-id"],
    [PveBridgeId, "pve-bridge-id"],
    [PveIface, "pve-iface"],
    [PveTaskStatusType, "pve-task-status-type"],
    [PveCommandBatch, "pve-command-batch"],
    [PveNode, "pve-node"],
    [PveVmId, "pve-vmid"],
    [PveVmCpuConf, "pve-vm-cpu-conf"],
    [PveHaResourceOrVmId, "pve-ha-resource-or-vm-id"],
    [PveCalendarEvent, "pve-calendar-event"],
    [PveHaGroupNode, "pve-ha-group-node"],
    [PveVolumeId, "pve-volume-id"],
    [PveHotplugFeatures, "pve-hotplug-features"],
    [PveStartupOrder, "pve-startup-order"],
    [PveTag, "pve-tag"],
    [PvePoolId, "pve-poolid"],
    [PveUserId, "pve-userid"],
    [PveGroupId, "pve-groupid"],
    [PveRoleId, "pve-roleid"],
    [PveTokenId, "pve-tokenid"],
    [PveConfigId, "pve-configid"],
    [PveStoragePortalDns, "pve-storage-portal-dns"],
    [PveStorageServer, "pve-storage-server"],
    [PveStorageId, "pve-storage-id"],
    [PveStorageContent, "pve-storage-content"],
    [PveStoragePath, "pve-storage-path"],
    [PveStorageFormat, "pve-storage-format"],
    [PveStorageOptions, "pve-storage-options"],
    [PveStorageVgName, "pve-storage-vgname"],
    [PveCpuSet, "pve-cpuset"],
    [PveQmBoot, "pve-qm-boot"],
    [PveQmBootDisk, "pve-qm-bootdisk"],
    [PveQmIde, "pve-qm-ide"],
    [PveQmCiCustom, "pve-qm-cicustom"],
    [PveQmHostpci, "pve-qm-hostpci"],
    [PveQmWatchdog, "pve-qm-watchdog"],
    [PveQmIpConfig, "pve-qm-ipconfig"],
    [PveQmSmbios1, "pve-qm-smbios1"],
    [PveQmUsbDevice, "pve-qm-usb-device"],
    [PveFwConntrackHelper, "pve-fw-conntrack-helper"],
    [PveFwSportSpec, "pve-fw-sport-spec"],
    [PveFwDportSpec, "pve-fw-dport-spec"],
    [PveFwAddrSpec, "pve-fw-addr-spec"],
    [PveFwProtocolSpec, "pve-fw-protocol-spec"],
    [PveFwIcmpTypeSpec, "pve-fw-icmp-type-spec"],
    [PveIpv4Config, "pve-ipv4-config"],
    [PveIpv6Config, "pve-ipv6-config"],
    [PveLxcDevString, "pve-lxc-dev-string"],

    [PveSdnSubnetId, "pve-sdn-subnet-id"],
    [PveSdnVnetId, "pve-sdn-vnet-id"],
    [PveSdnDhcpRange, "pve-sdn-dhcp-range"],
    [PveSdnZoneId, "pve-sdn-zone-id"],
    [PveSdnBgpRt, "pve-sdn-bgp-rt"],
    [PveSdnControllerId, "pve-sdn-controller-id"],
    [PveSdnIpamId, "pve-sdn-ipam-id"],
    [PveSdnDnsId, "pve-sdn-dns-id"],
);
