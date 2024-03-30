use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
};

use crate::raw::{self, Format, Items, Optional, ParametersOrU32};

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub optional: bool,
}

impl Type {
    pub fn parse(
        optional: &Optional,
        ty_str: &str,
        ty_text: Option<&Cow<str>>,
        enum_values: Option<&HashSet<Cow<str>>>,
        items: Option<&Items>,
        format: Option<&Format>,
        properties: Option<&BTreeMap<Cow<str>, raw::Property>>,
        allow_additional_properties: Option<bool>,
    ) -> Result<Self, String> {
        let kind = TypeKind::parse(
            ty_str,
            ty_text,
            enum_values,
            items,
            format,
            properties,
            allow_additional_properties,
        )?;

        Ok(Self {
            kind,
            optional: optional.get(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    String,
    Number,
    Integer,
    Boolean,
    Array(Box<Type>),
    Object {
        properties: BTreeMap<String, Type>,
        allow_additional_properties: bool,
    },
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

impl TypeKind {
    pub fn parse(
        ty_str: &str,
        ty_text: Option<&Cow<str>>,
        enum_values: Option<&HashSet<Cow<str>>>,
        items: Option<&Items>,
        format: Option<&Format>,
        properties: Option<&BTreeMap<Cow<str>, raw::Property>>,
        allow_additional_properties: Option<bool>,
    ) -> Result<Self, String> {
        let allow_additional_properties = allow_additional_properties.unwrap_or(false);

        let ty = match ty_str {
            "integer" => Self::Integer,
            "string" => Self::String,
            "number" => Self::Number,
            "boolean" => Self::Boolean,
            "object" => {
                return if let Some(properties) = properties {
                    let mut props = BTreeMap::new();
                    for (name, prop) in properties.iter() {
                        let ty = if let Some(ty) = prop.ty.as_ref() {
                            Type::parse(
                                &prop.optional,
                                ty,
                                prop.ty_text.as_ref(),
                                prop.enum_values.as_ref(),
                                prop.items.as_ref(),
                                prop.format.as_ref(),
                                prop.properties.as_ref(),
                                prop.additional_properties
                                    .as_ref()
                                    .map(ParametersOrU32::allow_additional),
                            )?
                        } else {
                            let kind = TypeKind::Object {
                                properties: Default::default(),
                                allow_additional_properties,
                            };

                            Type {
                                kind,
                                optional: prop.optional.get(),
                            }
                        };

                        props.insert(name.to_string(), ty);
                    }

                    Ok(Self::Object {
                        properties: props,
                        allow_additional_properties,
                    })
                } else {
                    Ok(Self::Object {
                        properties: BTreeMap::new(),
                        allow_additional_properties,
                    })
                };
            }
            "array" => {
                if let Some(items) = items {
                    let array_type = Type::parse(
                        &Optional::FALSE,
                        &items.ty,
                        None,
                        items.enum_values.as_ref(),
                        items.items.as_ref().map(|v| v.as_ref()),
                        items.format.as_ref(),
                        items.properties.as_ref(),
                        items.additional_properties.map(|v| v != 0),
                    )?;

                    return Ok(Self::Array(Box::new(array_type)));
                } else {
                    return Err(format!("Encountered array type without items."));
                }
            }
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
        } else if let (Self::Integer, Some(format)) = (&ty, format) {
            if format == "pve-vmid" {
                return Ok(Self::PveVmId);
            }
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
