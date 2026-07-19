use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tokio::sync::RwLock;

mod codex;
pub use codex::CodexProvider;

use crate::domain::{
    AppError, ConnectionType, ProviderCapabilities, ProviderId, ProviderMetadata, ProviderUsage,
    Reliability, UsagePeriod, UsageWarning, UsageWindow,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshReason {
    Manual,
    Startup,
    Scheduled,
    PopupOpen,
    Wake,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FetchContext {
    pub reason: RefreshReason,
}

impl FetchContext {
    pub fn manual() -> Self {
        Self {
            reason: RefreshReason::Manual,
        }
    }
}

#[async_trait]
pub trait ProviderConnector: Send + Sync {
    fn id(&self) -> ProviderId;
    fn label(&self) -> &'static str;
    fn capabilities(&self) -> ProviderCapabilities;
    async fn fetch_usage(&self, context: FetchContext) -> Result<ProviderUsage, AppError>;
    async fn start_login(&self) -> Result<crate::domain::LoginLaunchResult, AppError> {
        Err(AppError::Unsupported(
            "provider login is unavailable".into(),
        ))
    }
    async fn shutdown(&self) {}
}

#[derive(Clone)]
pub struct ProviderRegistry {
    providers: HashMap<ProviderId, Arc<dyn ProviderConnector>>,
    fake_scenarios: Option<Arc<RwLock<HashMap<ProviderId, FakeProviderScenario>>>>,
}

impl ProviderRegistry {
    pub fn new(connectors: Vec<Arc<dyn ProviderConnector>>) -> Self {
        let providers = connectors
            .into_iter()
            .map(|connector| (connector.id(), connector))
            .collect();
        Self {
            providers,
            fake_scenarios: None,
        }
    }

    pub fn fake() -> Self {
        Self::fake_with_scenario(FakeProviderScenario::ConnectedNormal)
    }

    pub fn production() -> Self {
        Self::new(vec![
            Arc::new(CodexProvider::production()) as Arc<dyn ProviderConnector>,
            Arc::new(UnimplementedProvider::new(ProviderId::Antigravity))
                as Arc<dyn ProviderConnector>,
        ])
    }

    pub fn fake_with_scenario(scenario: FakeProviderScenario) -> Self {
        let fake_scenarios = Arc::new(RwLock::new(
            ProviderId::ALL
                .into_iter()
                .map(|provider| (provider, scenario))
                .collect(),
        ));
        let providers = ProviderId::ALL
            .into_iter()
            .map(|provider| {
                (
                    provider,
                    Arc::new(FakeProvider::new(provider, fake_scenarios.clone()))
                        as Arc<dyn ProviderConnector>,
                )
            })
            .collect();
        Self {
            providers,
            fake_scenarios: Some(fake_scenarios),
        }
    }

    pub fn get(&self, provider: ProviderId) -> Option<Arc<dyn ProviderConnector>> {
        self.providers.get(&provider).cloned()
    }

    pub fn list_metadata(&self) -> Vec<ProviderMetadata> {
        ProviderId::ALL
            .into_iter()
            .filter_map(|provider| {
                self.providers
                    .get(&provider)
                    .map(|connector| ProviderMetadata {
                        id: provider,
                        label: connector.label().to_string(),
                        capabilities: connector.capabilities(),
                    })
            })
            .collect()
    }

    pub async fn fetch_usage(
        &self,
        provider: ProviderId,
        context: FetchContext,
    ) -> Result<ProviderUsage, AppError> {
        let connector = self.get(provider).ok_or_else(|| {
            AppError::ProviderUnavailable(format!("{provider} is not registered"))
        })?;
        connector.fetch_usage(context).await
    }

    pub async fn set_fake_scenario(
        &self,
        provider: ProviderId,
        scenario: FakeProviderScenario,
    ) -> Result<(), AppError> {
        let Some(fake_scenarios) = &self.fake_scenarios else {
            return Err(AppError::Unsupported(
                "fake scenarios are unavailable".into(),
            ));
        };
        fake_scenarios.write().await.insert(provider, scenario);
        Ok(())
    }

    pub async fn start_login(
        &self,
        provider: ProviderId,
    ) -> Result<crate::domain::LoginLaunchResult, AppError> {
        let connector = self.get(provider).ok_or_else(|| {
            AppError::ProviderUnavailable(format!("{provider} is not registered"))
        })?;
        connector.start_login().await
    }

    pub async fn shutdown_all(&self) {
        for connector in self.providers.values() {
            connector.shutdown().await;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FakeProviderScenario {
    ConnectedNormal,
    Warning,
    Critical,
    UnknownPercentage,
    Stale,
    Offline,
    AuthExpired,
    RetryableError,
}

struct FakeProvider {
    provider: ProviderId,
    scenarios: Arc<RwLock<HashMap<ProviderId, FakeProviderScenario>>>,
}

struct UnimplementedProvider {
    provider: ProviderId,
}

impl UnimplementedProvider {
    fn new(provider: ProviderId) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl ProviderConnector for UnimplementedProvider {
    fn id(&self) -> ProviderId {
        self.provider
    }

    fn label(&self) -> &'static str {
        match self.provider {
            ProviderId::Codex => "Codex",
            ProviderId::Antigravity => "Antigravity",
        }
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            can_detect_installation: false,
            can_detect_authentication: false,
            can_start_login: false,
            can_fetch_subscription_usage: false,
            can_fetch_api_usage: false,
            can_disconnect_local_connection: false,
            requires_tty: false,
            supports_multiple_windows: false,
            supports_model_windows: false,
        }
    }

    async fn fetch_usage(&self, _context: FetchContext) -> Result<ProviderUsage, AppError> {
        Err(AppError::ProviderUnavailable(format!(
            "{} integration is not implemented in M5",
            self.label()
        )))
    }
}

impl FakeProvider {
    fn new(
        provider: ProviderId,
        scenarios: Arc<RwLock<HashMap<ProviderId, FakeProviderScenario>>>,
    ) -> Self {
        Self {
            provider,
            scenarios,
        }
    }

    fn base_percent(&self, scenario: FakeProviderScenario) -> (Option<f64>, Option<f64>) {
        match scenario {
            FakeProviderScenario::Warning => (Some(84.0), Some(16.0)),
            FakeProviderScenario::Critical => (Some(96.0), Some(4.0)),
            FakeProviderScenario::UnknownPercentage => (None, None),
            _ => match self.provider {
                ProviderId::Codex => (Some(68.0), Some(32.0)),
                ProviderId::Antigravity => (Some(81.0), Some(19.0)),
            },
        }
    }
}

#[async_trait]
impl ProviderConnector for FakeProvider {
    fn id(&self) -> ProviderId {
        self.provider
    }

    fn label(&self) -> &'static str {
        match self.provider {
            ProviderId::Codex => "Codex",
            ProviderId::Antigravity => "Antigravity",
        }
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            can_detect_installation: true,
            can_detect_authentication: true,
            can_start_login: false,
            can_fetch_subscription_usage: true,
            can_fetch_api_usage: false,
            can_disconnect_local_connection: true,
            requires_tty: false,
            supports_multiple_windows: true,
            supports_model_windows: self.provider == ProviderId::Antigravity,
        }
    }

    async fn fetch_usage(&self, _context: FetchContext) -> Result<ProviderUsage, AppError> {
        let scenario = self
            .scenarios
            .read()
            .await
            .get(&self.provider)
            .copied()
            .unwrap_or(FakeProviderScenario::ConnectedNormal);

        match scenario {
            FakeProviderScenario::RetryableError => {
                return Err(AppError::RetryableProviderError(format!(
                    "Retryable usage check failed for {}",
                    self.provider
                )));
            }
            FakeProviderScenario::AuthExpired => return Err(AppError::AuthenticationExpired),
            _ => {}
        }

        let now = Utc::now();
        let (used, remaining) = self.base_percent(scenario);
        let mut windows = vec![
            UsageWindow::new("weekly", "Weekly limit", UsagePeriod::Weekly)
                .and_then(|window| window.with_percentages(used, remaining))
                .map_err(|_| AppError::Internal)?,
            UsageWindow::new("session", "Session usage", UsagePeriod::Session)
                .and_then(|window| window.with_percentages(Some(35.0), Some(65.0)))
                .map_err(|_| AppError::Internal)?,
        ];

        if self.provider == ProviderId::Antigravity {
            windows[0] = windows[0].clone().with_model("Gemini Pro");
        }

        let mut usage = ProviderUsage::new(
            self.provider,
            match self.provider {
                ProviderId::Antigravity => ConnectionType::OAuthCli,
                _ => ConnectionType::SubscriptionCli,
            },
            Reliability::OfficialCliText,
            windows,
            now,
        )
        .with_account("fake@example.local", "Fake Provider Plan");

        match scenario {
            FakeProviderScenario::UnknownPercentage => {
                usage.warnings.push(UsageWarning::PercentageUnavailable);
            }
            FakeProviderScenario::Stale => {
                usage.stale = true;
                usage.warnings.push(UsageWarning::Stale);
            }
            FakeProviderScenario::Offline => {
                usage.stale = true;
                usage.warnings.push(UsageWarning::Offline);
            }
            _ => {}
        }

        Ok(usage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{AppError, ProviderId};

    #[tokio::test]
    async fn fake_registry_registers_all_providers() {
        let registry = ProviderRegistry::fake();
        let providers = registry.list_metadata();

        assert_eq!(providers.len(), 2);
        assert!(registry.get(ProviderId::Codex).is_some());
        assert!(registry.get(ProviderId::Antigravity).is_some());
    }

    #[tokio::test]
    async fn fake_registry_preserves_unknown_percentages() {
        let registry =
            ProviderRegistry::fake_with_scenario(FakeProviderScenario::UnknownPercentage);
        let usage = registry
            .fetch_usage(ProviderId::Codex, FetchContext::manual())
            .await
            .expect("fake usage");

        assert_eq!(usage.windows[0].used_percent, None);
    }

    #[tokio::test]
    async fn fake_registry_isolates_provider_failures() {
        let registry = ProviderRegistry::fake_with_scenario(FakeProviderScenario::RetryableError);

        assert!(matches!(
            registry
                .fetch_usage(ProviderId::Codex, FetchContext::manual())
                .await,
            Err(AppError::RetryableProviderError(_))
        ));
        assert_eq!(registry.list_metadata().len(), 2);
    }
}
