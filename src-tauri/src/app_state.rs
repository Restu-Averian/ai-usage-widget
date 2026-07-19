use std::sync::Arc;

use crate::database::Database;
use crate::domain::{AppError, AppErrorPayload};
use crate::events::{BackendEventEmitter, TauriBackendEventEmitter};
#[cfg(test)]
use crate::events::{MemoryBackendEventEmitter, SharedMemoryEventEmitter};
use crate::providers::ProviderRegistry;
use crate::scheduler::RefreshCoordinator;
#[cfg(test)]
use crate::secrets::MemorySecretStore;
use crate::secrets::{NativeSecretStore, SecretStore};
use tokio::sync::{Notify, OnceCell};

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
            ProviderRegistry::production(),
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

#[derive(Default)]
pub struct AppStateHandle {
    state: OnceCell<Result<AppState, AppErrorPayload>>,
    ready: Notify,
}

impl AppStateHandle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ready(&self) -> bool {
        self.state.get().is_some()
    }

    pub async fn initialize_production(
        &self,
        database_path: impl AsRef<std::path::Path>,
        app: tauri::AppHandle,
    ) {
        eprintln!("[startup] initializing application state");
        let database_path = database_path.as_ref();
        let result = match database_path.parent() {
            Some(parent) => match std::fs::create_dir_all(parent) {
                Ok(()) => AppState::production(database_path, app)
                    .await
                    .map_err(|error| error.payload()),
                Err(error) => {
                    eprintln!(
                        "[startup] app data directory initialization failed: {}",
                        error.kind()
                    );
                    Err(AppError::Database.payload())
                }
            },
            None => Err(AppError::Database.payload()),
        };
        match &result {
            Ok(_) => eprintln!("[startup] application state initialized"),
            Err(error) => eprintln!(
                "[startup] application state initialization failed: {:?}",
                error.code
            ),
        }
        let _ = self.state.set(result);
        self.ready.notify_waiters();
    }

    pub fn initialize_failed(&self, error: AppErrorPayload) {
        eprintln!(
            "[startup] application state initialization failed: {:?}",
            error.code
        );
        let _ = self.state.set(Err(error));
        self.ready.notify_waiters();
    }

    pub async fn get(&self) -> Result<AppState, AppErrorPayload> {
        loop {
            if let Some(result) = self.state.get() {
                return result.clone();
            }
            self.ready.notified().await;
        }
    }

    pub fn scheduler_if_ready(&self) -> Option<Arc<RefreshCoordinator>> {
        self.state
            .get()
            .and_then(|result| result.as_ref().ok())
            .map(|state| state.scheduler.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn startup_handle_can_be_managed_before_state_is_ready() {
        let handle = AppStateHandle::new();

        assert!(!handle.is_ready());
    }

    #[tokio::test]
    async fn test_state_uses_fake_dependencies() {
        let state = AppState::test().await.expect("state");

        assert_eq!(state.providers.list_metadata().len(), 3);
    }
}
