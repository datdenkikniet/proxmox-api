#[derive(Debug, Clone)]
pub struct StateClient<T> {
    client: T,
    path: String,
}
impl<T> StateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/state"),
        }
    }
}
impl<T> StateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read service properties"]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutput {
    pub fn new(
        active_state: ActiveState,
        desc: String,
        name: String,
        service: String,
        state: State,
        unit_state: UnitState,
    ) -> Self {
        Self {
            active_state,
            desc,
            name,
            service,
            state,
            unit_state,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(rename = "active-state")]
    #[doc = "Current state of the service process (systemd ActiveState)."]
    #[doc = ""]
    pub active_state: ActiveState,
    #[doc = "Description of the service."]
    #[doc = ""]
    pub desc: String,
    #[doc = "Short identifier for the service (e.g., \"pveproxy\")."]
    #[doc = ""]
    pub name: String,
    #[doc = "Systemd unit name (e.g., pveproxy)."]
    #[doc = ""]
    pub service: String,
    #[doc = "Execution status of the service (systemd SubState)."]
    #[doc = ""]
    pub state: State,
    #[serde(rename = "unit-state")]
    #[doc = "Whether the service is enabled (systemd UnitFileState)."]
    #[doc = ""]
    pub unit_state: UnitState,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Current state of the service process (systemd ActiveState)."]
#[doc = ""]
pub enum ActiveState {
    #[serde(rename = "activating")]
    Activating,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deactivating")]
    Deactivating,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "maintenance")]
    Maintenance,
    #[serde(rename = "refreshing")]
    Refreshing,
    #[serde(rename = "reloading")]
    Reloading,
    #[serde(rename = "unknown")]
    Unknown,
}
impl TryFrom<&str> for ActiveState {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "activating" => Ok(Self::Activating),
            "active" => Ok(Self::Active),
            "deactivating" => Ok(Self::Deactivating),
            "failed" => Ok(Self::Failed),
            "inactive" => Ok(Self::Inactive),
            "maintenance" => Ok(Self::Maintenance),
            "refreshing" => Ok(Self::Refreshing),
            "reloading" => Ok(Self::Reloading),
            "unknown" => Ok(Self::Unknown),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Execution status of the service (systemd SubState)."]
#[doc = ""]
pub enum State {
    #[serde(rename = "auto-restart")]
    AutoRestart,
    #[serde(rename = "auto-restart-queued")]
    AutoRestartQueued,
    #[serde(rename = "cleaning")]
    Cleaning,
    #[serde(rename = "condition")]
    Condition,
    #[serde(rename = "dead")]
    Dead,
    #[serde(rename = "dead-before-auto-restart")]
    DeadBeforeAutoRestart,
    #[serde(rename = "dead-resources-pinned")]
    DeadResourcesPinned,
    #[serde(rename = "exited")]
    Exited,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "failed-before-auto-restart")]
    FailedBeforeAutoRestart,
    #[serde(rename = "final-sigkill")]
    FinalSigkill,
    #[serde(rename = "final-sigterm")]
    FinalSigterm,
    #[serde(rename = "final-watchdog")]
    FinalWatchdog,
    #[serde(rename = "mounting")]
    Mounting,
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "reload-notify")]
    ReloadNotify,
    #[serde(rename = "reload-signal")]
    ReloadSignal,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "start-post")]
    StartPost,
    #[serde(rename = "start-pre")]
    StartPre,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "stop-post")]
    StopPost,
    #[serde(rename = "stop-sigkill")]
    StopSigkill,
    #[serde(rename = "stop-sigterm")]
    StopSigterm,
    #[serde(rename = "stop-watchdog")]
    StopWatchdog,
    #[serde(rename = "unknown")]
    Unknown,
}
impl TryFrom<&str> for State {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "auto-restart" => Ok(Self::AutoRestart),
            "auto-restart-queued" => Ok(Self::AutoRestartQueued),
            "cleaning" => Ok(Self::Cleaning),
            "condition" => Ok(Self::Condition),
            "dead" => Ok(Self::Dead),
            "dead-before-auto-restart" => Ok(Self::DeadBeforeAutoRestart),
            "dead-resources-pinned" => Ok(Self::DeadResourcesPinned),
            "exited" => Ok(Self::Exited),
            "failed" => Ok(Self::Failed),
            "failed-before-auto-restart" => Ok(Self::FailedBeforeAutoRestart),
            "final-sigkill" => Ok(Self::FinalSigkill),
            "final-sigterm" => Ok(Self::FinalSigterm),
            "final-watchdog" => Ok(Self::FinalWatchdog),
            "mounting" => Ok(Self::Mounting),
            "reload" => Ok(Self::Reload),
            "reload-notify" => Ok(Self::ReloadNotify),
            "reload-signal" => Ok(Self::ReloadSignal),
            "running" => Ok(Self::Running),
            "start" => Ok(Self::Start),
            "start-post" => Ok(Self::StartPost),
            "start-pre" => Ok(Self::StartPre),
            "stop" => Ok(Self::Stop),
            "stop-post" => Ok(Self::StopPost),
            "stop-sigkill" => Ok(Self::StopSigkill),
            "stop-sigterm" => Ok(Self::StopSigterm),
            "stop-watchdog" => Ok(Self::StopWatchdog),
            "unknown" => Ok(Self::Unknown),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Whether the service is enabled (systemd UnitFileState)."]
#[doc = ""]
pub enum UnitState {
    #[serde(rename = "alias")]
    Alias,
    #[serde(rename = "bad")]
    Bad,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "enabled-runtime")]
    EnabledRuntime,
    #[serde(rename = "generated")]
    Generated,
    #[serde(rename = "indirect")]
    Indirect,
    #[serde(rename = "linked")]
    Linked,
    #[serde(rename = "linked-runtime")]
    LinkedRuntime,
    #[serde(rename = "masked")]
    Masked,
    #[serde(rename = "masked-runtime")]
    MaskedRuntime,
    #[serde(rename = "not-found")]
    NotFound,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "transient")]
    Transient,
    #[serde(rename = "unknown")]
    Unknown,
}
impl TryFrom<&str> for UnitState {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "alias" => Ok(Self::Alias),
            "bad" => Ok(Self::Bad),
            "disabled" => Ok(Self::Disabled),
            "enabled" => Ok(Self::Enabled),
            "enabled-runtime" => Ok(Self::EnabledRuntime),
            "generated" => Ok(Self::Generated),
            "indirect" => Ok(Self::Indirect),
            "linked" => Ok(Self::Linked),
            "linked-runtime" => Ok(Self::LinkedRuntime),
            "masked" => Ok(Self::Masked),
            "masked-runtime" => Ok(Self::MaskedRuntime),
            "not-found" => Ok(Self::NotFound),
            "static" => Ok(Self::Static),
            "transient" => Ok(Self::Transient),
            "unknown" => Ok(Self::Unknown),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
