use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{AppErrorPayload, ProviderId, Reliability};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionType {
    SubscriptionCli,
    OAuthCli,
    ApiKey,
    AdminApiKey,
    CloudProject,
    DashboardOnly,
}

impl ConnectionType {
    pub fn as_str(self) -> &'static str {
        match self {
            ConnectionType::SubscriptionCli => "subscription-cli",
            ConnectionType::OAuthCli => "oauth-cli",
            ConnectionType::ApiKey => "api-key",
            ConnectionType::AdminApiKey => "admin-api-key",
            ConnectionType::CloudProject => "cloud-project",
            ConnectionType::DashboardOnly => "dashboard-only",
        }
    }
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderStateKind {
    Unknown,
    Detecting,
    NotInstalled,
    AuthenticationRequired,
    Connecting,
    Connected,
    Refreshing,
    Stale,
    Offline,
    Error,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConnection {
    pub id: String,
    pub provider: ProviderId,
    pub connection_type: ConnectionType,
    pub account_label: Option<String>,
    pub plan_name: Option<String>,
    pub status: ProviderStateKind,
    pub executable_path: Option<String>,
    pub cli_version: Option<String>,
    pub reliability: Option<Reliability>,
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub last_error: Option<AppErrorPayload>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
