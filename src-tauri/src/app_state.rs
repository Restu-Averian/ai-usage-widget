use std::sync::Arc;

use crate::database::Database;
use crate::domain::AppError;
use crate::events::{BackendEventEmitter, TauriBackendEventEmitter};
#[cfg(test)]
use crate::events::{MemoryBackendEventEmitter, SharedMemoryEventEmitter};
use crate::providers::ProviderRegistry;
use crate::scheduler::RefreshCoordinator;
#[cfg(test)]
use crate::secrets::MemorySecretStore;
use crate::secrets::{NativeSecretStore, SecretStore};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub providers: ProviderRegistry,
    pub secrets: Arc<dyn SecretStore>,
    pub scheduler: Arc<RefreshCoordinator>,
    pub events: Arc<dyn BackendEventEmitter>,
}

impl AppState {
    pub fn new(
        database: Database,
        providers: ProviderRegistry,
        secrets: Arc<dyn SecretStore>,
        events: Arc<dyn BackendEventEmitter>,
    ) -> Self {
        Self {
            database,
            providers,
            secrets,
            scheduler: Arc::new(RefreshCoordinator::new()),
            events,
        }
    }

    pub async fn production(
        database_path: impl AsRef<std::path::Path>,
        app: tauri::AppHandle,
    ) -> Result<Self, AppError> {
        Ok(Self::new(
            Database::connect(database_path).await?,
            ProviderRegistry::fake(),
            Arc::new(NativeSecretStore),
            Arc::new(TauriBackendEventEmitter::new(app)),
        ))
    }

    #[cfg(test)]
    pub async fn test() -> Result<Self, AppError> {
        Self::test_with_registry(ProviderRegistry::fake()).await
    }

    #[cfg(test)]
    pub async fn test_with_registry(providers: ProviderRegistry) -> Result<Self, AppError> {
        Ok(Self::new(
            Database::in_memory().await?,
            providers,
            Arc::new(MemorySecretStore::default()),
            Arc::new(MemoryBackendEventEmitter::default()),
        ))
    }

    #[cfg(test)]
    pub async fn test_with_memory_events() -> Result<(Self, SharedMemoryEventEmitter), AppError> {
        let events = Arc::new(MemoryBackendEventEmitter::default());
        Ok((
            Self::new(
                Database::in_memory().await?,
                ProviderRegistry::fake(),
                Arc::new(MemorySecretStore::default()),
                events.clone(),
            ),
            events,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_state_uses_fake_dependencies() {
        let state = AppState::test().await.expect("state");

        assert_eq!(state.providers.list_metadata().len(), 3);
    }
}
