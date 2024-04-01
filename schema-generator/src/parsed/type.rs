use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use crate::raw::{Format, Items};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Number,
    Integer,
    Boolean,
    Array(Box<Type>),
    Object(HashMap<String, Type>),
    Enum(HashSet<String>),

    IPorCIDRorAlias,
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

impl Type {
    fn parse_format(format: &Format) -> Result<HashMap<String, Type>, String> {
        todo!()
    }

    pub fn parse(
        ty_str: &str,
        ty_text: Option<&Cow<str>>,
        enum_values: Option<&HashSet<Cow<str>>>,
        items: Option<&Items>,
        format: Option<&Format>,
    ) -> Result<Self, String> {
        let ty = match ty_str {
            "integer" => Self::Integer,
            "string" => Self::String,
            "number" => Self::Number,
            "boolean" => Self::Boolean,
            "object" => {
                if let Some(format) = format {
                    let format = Self::parse_format(format)?;
                    return Ok(Self::Object(format));
                } else {
                    return Err(format!("Object type without format"));
                }
            }
            "array" => return Err(format!("Cannot parse {ty_str} correctly yet.")),
            _ => return Err(format!("Unknown type {ty_str}")),
        };

        if let (Self::String, Some(format)) = (&ty, format) {
            let value = if format == "pve-priv-list" {
                Self::PvePrivList
            } else if format == "pve-realm" {
                Self::PveRealm
            } else if format == "pve-userid" {
                Self::PveUserId
            } else if format == "pve-poolid" {
                Self::PvePoolId
            } else if format == "pve-replication-job-id" {
                Self::PveReplicationJobId
            } else if format == "pve-configid" {
                Self::PveConfigId
            } else if format == "pve-node" {
                Self::PveNode
            } else if format == "pve-ha-resource-or-vm-id" {
                Self::PveHaResourceOrVmId
            } else if format == "pve-calendar-event" {
                Self::PveCalendarEvent
            } else if format == "pve-configid-list" {
                Self::PveConfigIdList
            } else if format == "pve-storage-id" {
                Self::PveStorageId
            } else if format == "pve-bridge-id-list" {
                Self::PveBridgeIdList
            } else if format == "pve-storage-id-list" {
                Self::PveStorageIdList
            } else if format == "pve-storage-server" {
                Self::PveStorageServer
            } else if format == "pve-storage-content" {
                Self::PveStorageContent
            } else if format == "pve-storage-content-list" {
                Self::PveStorageContentList
            } else if format == "pve-vmid-list" {
                Self::PveVmidList
            } else if format == "pve-iface" {
                Self::PveIface
            } else if format == "pve-groupid" {
                Self::PveGroupId
            } else if format == "pve-task-status-type-list" {
                Self::PveTaskStatusTypeList
            } else if format == "pve-storage-portal-dns" {
                Self::PveStoragePortalDns
            } else if format == "pve-command-batch" {
                Self::PveCommandBatch
            } else if format == "pve-roleid" {
                Self::PveRoleId
            } else if format == "proxmox-remote" {
                Self::ProxmoxRemote
            } else if format == "IPorCIDRorAlias" {
                Self::IPorCIDRorAlias
            } else if format == "string-alist" {
                Self::ArgumentList
            } else if format == "address" {
                Self::InternetAddress
            } else if format == "cidr" || format == "CIDR" {
                Self::Cidr
            } else if format == "storage-pair-list" {
                Self::StoragePairList
            } else if format == "dns-name" {
                Self::DnsName
            } else if format == "bridge-pair-list" {
                Self::BridgePairList
            } else if format == "email-or-username-list" {
                Self::EmailOrUsernameList
            } else if format == "backup-performance" {
                Self::BackupPerformance
            } else if format == "prune-backups" {
                Self::PruneBackups
            } else if format == "string-list" {
                Self::StringList
            } else {
                return Err(format!("Unknown string format {format:?}"));
            };

            return Ok(value);
        }

        if let Some(enum_values) = enum_values {
            return Ok(Self::Enum(enum_values.iter().map(Cow::to_string).collect()));
        }

        if let Some(format) = format {
            return Err(format!(
                "Do not know of type {ty_str} with format {format:?}."
            ));
        }

        Ok(ty)
    }
}
