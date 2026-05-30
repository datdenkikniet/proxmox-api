use proc_macro2::Literal;
use serde::de;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Permission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<Check>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Check {
    Perm {
        path: String,
        privileges: Vec<String>,
        #[serde(default)]
        any: bool,
        #[serde(default)]
        require_param: Option<String>,
    },
    PermModify {
        path: String,
    },
    UserIdGroup {
        privileges: Vec<String>,
        #[serde(default)]
        groups_param: Option<String>,
    },
    UserIdParam {
        privilege: String,
    },
    Or(Vec<Check>),
    And(Vec<Check>),
}

impl std::fmt::Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Check::Perm {
                path,
                privileges,
                any,
                require_param,
            } => {
                let privs = privileges
                    .iter()
                    .map(|s| format!("\"{s}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "perm(\"{path}\", [{privs}]")?;
                if *any {
                    write!(f, ", any")?;
                }
                if let Some(param) = require_param {
                    write!(f, ", require_param=\"{param}\"")?;
                }
                write!(f, ")")
            }
            Check::PermModify { path } => {
                write!(f, "perm-modify(\"{}\")", path)
            }
            Check::UserIdGroup {
                privileges,
                groups_param,
            } => {
                let privs = privileges
                    .iter()
                    .map(|s| format!("\"{s}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "userid-group([{privs}]")?;
                if let Some(param) = groups_param {
                    write!(f, ", groups_param=\"{param}\"")?;
                }
                write!(f, ")")
            }
            Check::UserIdParam { privilege } => {
                write!(f, "userid-param(\"{}\")", privilege)
            }
            Check::Or(checks) | Check::And(checks) => {
                let name = if matches!(self, Check::Or(_)) {
                    "or"
                } else {
                    "and"
                };
                let checks = checks
                    .iter()
                    .map(Check::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{name}({checks})")
            }
        }
    }
}

impl Permission {
    pub fn doc_strings(&self) -> Vec<Literal> {
        if self.user.as_deref() == Some("world") {
            return vec![Literal::string("Accessible without authentication.")];
        }
        let mut lines = Vec::new();
        if let Some(ref check) = self.check {
            let check_line = format!("Permission check: {check}");
            lines.push(Literal::string(&check_line));
        }
        if let Some(ref desc) = self.description {
            let cleaned = desc
                .replace('<', "\\<")
                .replace('>', "\\>")
                .replace('[', "\\[")
                .replace(']', "\\]");
            lines.push(Literal::string(&cleaned));
        }
        lines
    }
}

fn parse_privileges(values: &[serde_json::Value]) -> Result<Vec<String>, String> {
    values
        .iter()
        .map(|v| {
            v.as_str()
                .ok_or_else(|| "expected string privilege".to_string())
                .map(|s| s.to_string())
        })
        .collect()
}

impl Check {
    fn from_value(value: &serde_json::Value) -> Result<Self, String> {
        let arr = value.as_array().ok_or("expected array for check")?;
        let Some((tag, arr)) = arr.split_first() else {
            return Err("empty check array".to_string());
        };
        let tag = tag
            .as_str()
            .ok_or("expected string tag as first element of check array")?;

        match tag {
            "perm" => {
                if arr.len() < 2 {
                    return Err("perm check requires at least 3 elements".to_string());
                }
                let path = arr[0]
                    .as_str()
                    .ok_or("expected string path for perm check")?
                    .to_string();
                let privileges = parse_privileges(
                    arr[1]
                        .as_array()
                        .ok_or("expected array of privileges for perm check")?,
                )?;

                let mut any = false;
                let mut require_param: Option<String> = None;

                let (chunks, remainder) = arr[2..].as_chunks::<2>();
                if let [key] = remainder {
                    let key = key.as_str().unwrap_or("?");
                    return Err(format!("expected value for '{key}' in perm options"));
                }

                for [key, val] in chunks {
                    let key = key
                        .as_str()
                        .ok_or_else(|| "expected string key in perm options".to_string())?;
                    match key {
                        "any" => {
                            any = val.as_u64() == Some(1);
                        }
                        "require_param" => {
                            require_param = Some(
                                val.as_str()
                                    .ok_or_else(|| {
                                        "expected string for 'require_param'".to_string()
                                    })?
                                    .to_string(),
                            );
                        }
                        _ => return Err(format!("unknown perm option: {key}")),
                    }
                }

                Ok(Check::Perm {
                    path,
                    privileges,
                    any,
                    require_param,
                })
            }
            "perm-modify" => {
                if arr.is_empty() {
                    return Err("perm-modify check requires at least 2 elements".to_string());
                }
                let path = arr[0]
                    .as_str()
                    .ok_or("expected string path for perm-modify check")?
                    .to_string();
                Ok(Check::PermModify { path })
            }
            "userid-group" => {
                if arr.is_empty() {
                    return Err("userid-group check requires at least 2 elements".to_string());
                }
                let privileges = parse_privileges(
                    arr[0]
                        .as_array()
                        .ok_or("expected array of privileges for userid-group check")?,
                )?;

                let mut groups_param: Option<String> = None;

                let (chunks, remainder) = arr[1..].as_chunks::<2>();
                if let [key] = remainder {
                    let key = key.as_str().unwrap_or("?");
                    return Err(format!(
                        "expected value for '{key}' in userid-group options"
                    ));
                }
                for [key, val] in chunks {
                    let key = key
                        .as_str()
                        .ok_or_else(|| "expected string key in userid-group options".to_string())?;
                    match key {
                        "groups_param" => {
                            groups_param = Some(
                                val.as_str()
                                    .ok_or_else(|| {
                                        "expected string for 'groups_param'".to_string()
                                    })?
                                    .to_string(),
                            );
                        }
                        _ => return Err(format!("unknown userid-group option: {key}")),
                    }
                }

                Ok(Check::UserIdGroup {
                    privileges,
                    groups_param,
                })
            }
            "userid-param" => {
                if arr.is_empty() {
                    return Err("userid-param check requires at least 2 elements".to_string());
                }
                let privilege = arr[0]
                    .as_str()
                    .ok_or("expected string privilege for userid-param check")?
                    .to_string();
                Ok(Check::UserIdParam { privilege })
            }
            "or" | "and" => {
                let checks = arr
                    .iter()
                    .map(Check::from_value)
                    .collect::<Result<Vec<_>, _>>()?;
                if checks.is_empty() {
                    return Err(format!("{tag} check requires at least one sub-check"));
                }
                Ok(if tag == "or" {
                    Check::Or(checks)
                } else {
                    Check::And(checks)
                })
            }
            _ => Err(format!("unknown check type: {tag}")),
        }
    }
}

impl<'de> Deserialize<'de> for Check {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        Check::from_value(&value).map_err(de::Error::custom)
    }
}
