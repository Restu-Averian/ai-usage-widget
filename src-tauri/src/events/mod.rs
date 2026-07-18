use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::Serialize;
use tauri::Emitter;

use crate::domain::{AppError, AppErrorPayload, ProviderId};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum BackendEvent {
    ProviderRefreshStarted {
        provider: ProviderId,
    },
    ProviderRefreshCompleted {
        provider: ProviderId,
        fetched_at: DateTime<Utc>,
        changed: bool,
    },
    ProviderRefreshFailed {
        provider: ProviderId,
        error: AppErrorPayload,
    },
    SettingsChanged,
    Wake,
    NetworkStateChanged,
}

impl BackendEvent {
    pub fn name(&self) -> &'static str {
        match self {
            BackendEvent::ProviderRefreshStarted { .. } => "provider://refresh-started",
            BackendEvent::ProviderRefreshCompleted { .. } => "provider://refresh-completed",
            BackendEvent::ProviderRefreshFailed { .. } => "provider://refresh-failed",
            BackendEvent::SettingsChanged => "settings://changed",
            BackendEvent::Wake => "app://wake",
            BackendEvent::NetworkStateChanged => "app://network-state-changed",
        }
    }
}

pub trait BackendEventEmitter: Send + Sync {
    fn emit(&self, event: BackendEvent) -> Result<(), AppError>;
}

#[derive(Default)]
pub struct MemoryBackendEventEmitter {
    events: Mutex<Vec<BackendEvent>>,
}

impl MemoryBackendEventEmitter {
    pub fn events(&self) -> Vec<BackendEvent> {
        self.events.lock().expect("event lock").clone()
    }
}

impl BackendEventEmitter for MemoryBackendEventEmitter {
    fn emit(&self, event: BackendEvent) -> Result<(), AppError> {
        self.events.lock().expect("event lock").push(event);
        Ok(())
    }
}

#[derive(Clone)]
pub struct TauriBackendEventEmitter {
    app: tauri::AppHandle,
}

impl TauriBackendEventEmitter {
    pub fn new(app: tauri::AppHandle) -> Self {
        Self { app }
    }
}

impl BackendEventEmitter for TauriBackendEventEmitter {
    fn emit(&self, event: BackendEvent) -> Result<(), AppError> {
        self.app
            .emit(event.name(), &event)
            .map_err(|_| AppError::Internal)
    }
}

pub type SharedMemoryEventEmitter = Arc<MemoryBackendEventEmitter>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_event_emitter_records_typed_events() {
        let emitter = MemoryBackendEventEmitter::default();
        emitter
            .emit(BackendEvent::ProviderRefreshStarted {
                provider: crate::domain::ProviderId::Codex,
            })
            .expect("emit");

        assert_eq!(emitter.events().len(), 1);
        assert_eq!(emitter.events()[0].name(), "provider://refresh-started");
    }
}
