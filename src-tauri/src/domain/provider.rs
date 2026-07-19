use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderId {
    Codex,
    Claude,
    Antigravity,
}

impl ProviderId {
    pub const ALL: [ProviderId; 3] = [
        ProviderId::Codex,
        ProviderId::Claude,
        ProviderId::Antigravity,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            ProviderId::Codex => "codex",
            ProviderId::Claude => "claude",
            ProviderId::Antigravity => "antigravity",
        }
    }
}

impl fmt::Display for ProviderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Reliability {
    OfficialApi,
    OfficialSdk,
    OfficialCliJson,
    OfficialCliText,
    ExperimentalPty,
    Manual,
    DashboardOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCapabilities {
    pub can_detect_installation: bool,
    pub can_detect_authentication: bool,
    pub can_start_login: bool,
    pub can_fetch_subscription_usage: bool,
    pub can_fetch_api_usage: bool,
    pub can_disconnect_local_connection: bool,
    pub requires_tty: bool,
    pub supports_multiple_windows: bool,
    pub supports_model_windows: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderMetadata {
    pub id: ProviderId,
    pub label: String,
    pub capabilities: ProviderCapabilities,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InstallationTrust {
    Detected,
    Approved,
    PathChanged,
    VersionUnsupported,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallationStatus {
    pub installed: bool,
    pub executable_path: Option<String>,
    pub version: Option<String>,
    pub trusted: InstallationTrust,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuthenticationStatus {
    Unknown,
    Authenticated,
    Required,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginLaunchResult {
    pub launched: bool,
    pub message: Option<String>,
}
