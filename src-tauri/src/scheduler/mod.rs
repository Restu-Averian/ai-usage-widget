use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use tokio::sync::{Mutex, Notify};

use crate::app_state::AppState;
use crate::commands::ProviderState;
use crate::domain::{AppError, ProviderId, ProviderStateKind};
use crate::events::BackendEvent;
use crate::providers::FetchContext;

#[derive(Default)]
pub struct RefreshCoordinator {
    in_flight: Mutex<HashMap<ProviderId, Arc<Notify>>>,
    fetch_counts: Mutex<HashMap<ProviderId, usize>>,
    shutdown: Mutex<bool>,
}

impl RefreshCoordinator {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn shutdown(&self) {
        *self.shutdown.lock().await = true;
    }

    pub async fn refresh_provider(
        &self,
        state: &AppState,
        provider: ProviderId,
    ) -> Result<ProviderState, AppError> {
        if *self.shutdown.lock().await {
            return state_from_cache(state, provider).await;
        }

        let (notify, leader) = {
            let mut in_flight = self.in_flight.lock().await;
            if let Some(notify) = in_flight.get(&provider) {
                (notify.clone(), false)
            } else {
                let notify = Arc::new(Notify::new());
                in_flight.insert(provider, notify.clone());
                (notify, true)
            }
        };

        if !leader {
            notify.notified().await;
            return state_from_cache(state, provider).await;
        }

        let result = self.refresh_provider_inner(state, provider).await;
        self.in_flight.lock().await.remove(&provider);
        notify.notify_waiters();
        result
    }

    async fn refresh_provider_inner(
        &self,
        state: &AppState,
        provider: ProviderId,
    ) -> Result<ProviderState, AppError> {
        *self.fetch_counts.lock().await.entry(provider).or_default() += 1;
        state
            .events
            .emit(BackendEvent::ProviderRefreshStarted { provider })?;

        match state
            .providers
            .fetch_usage(provider, FetchContext::manual())
            .await
        {
            Ok(usage) => {
                state.database.save_usage_snapshot(&usage).await?;
                state.database.upsert_connection_from_usage(&usage).await?;
                state.events.emit(BackendEvent::ProviderRefreshCompleted {
                    provider,
                    fetched_at: usage.fetched_at,
                    changed: true,
                })?;
                Ok(ProviderState {
                    provider,
                    status: if usage.stale {
                        ProviderStateKind::Stale
                    } else {
                        ProviderStateKind::Connected
                    },
                    usage: Some(usage),
                    last_error: None,
                    is_refreshing: false,
                })
            }
            Err(error) => match state.database.latest_usage(provider).await? {
                Some(mut usage) => {
                    usage.stale = true;
                    let payload = error.payload();
                    state.events.emit(BackendEvent::ProviderRefreshFailed {
                        provider,
                        error: payload.clone(),
                    })?;
                    Ok(ProviderState {
                        provider,
                        status: ProviderStateKind::Stale,
                        usage: Some(usage),
                        last_error: Some(payload),
                        is_refreshing: false,
                    })
                }
                None => {
                    state.events.emit(BackendEvent::ProviderRefreshFailed {
                        provider,
                        error: error.payload(),
                    })?;
                    Err(error)
                }
            },
        }
    }

    #[cfg(test)]
    pub async fn provider_fetch_count(&self, provider: ProviderId) -> usize {
        self.fetch_counts
            .lock()
            .await
            .get(&provider)
            .copied()
            .unwrap_or(0)
    }
}

pub fn is_stale(fetched_at: DateTime<Utc>, interval_minutes: i64) -> bool {
    Utc::now() - fetched_at > ChronoDuration::minutes(interval_minutes * 2)
}

async fn state_from_cache(
    state: &AppState,
    provider: ProviderId,
) -> Result<ProviderState, AppError> {
    let usage = state.database.latest_usage(provider).await?;
    Ok(ProviderState {
        provider,
        status: usage
            .as_ref()
            .map(|usage| {
                if usage.stale {
                    ProviderStateKind::Stale
                } else {
                    ProviderStateKind::Connected
                }
            })
            .unwrap_or(ProviderStateKind::Unknown),
        usage,
        last_error: None,
        is_refreshing: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ProviderId, ProviderStateKind};
    use crate::providers::{FakeProviderScenario, ProviderRegistry};
    use chrono::{Duration as ChronoDuration, Utc};

    #[test]
    fn marks_snapshot_stale_after_two_refresh_intervals() {
        let fetched_at = Utc::now() - ChronoDuration::minutes(11);

        assert!(is_stale(fetched_at, 5));
    }

    #[tokio::test]
    async fn coalesces_duplicate_provider_refreshes() {
        let state = crate::app_state::AppState::test_with_registry(
            ProviderRegistry::fake_with_scenario(FakeProviderScenario::ConnectedNormal),
        )
        .await
        .expect("state");
        let coordinator = RefreshCoordinator::new();

        let (first, second) = tokio::join!(
            coordinator.refresh_provider(&state, ProviderId::Codex),
            coordinator.refresh_provider(&state, ProviderId::Codex)
        );

        assert_eq!(first.expect("first").status, ProviderStateKind::Connected);
        assert_eq!(second.expect("second").status, ProviderStateKind::Connected);
        assert_eq!(coordinator.provider_fetch_count(ProviderId::Codex).await, 1);
    }

    #[tokio::test]
    async fn emits_refresh_events() {
        let (state, events) = crate::app_state::AppState::test_with_memory_events()
            .await
            .expect("state");
        let coordinator = RefreshCoordinator::new();

        coordinator
            .refresh_provider(&state, ProviderId::Codex)
            .await
            .expect("refresh");

        let names: Vec<_> = events.events().iter().map(|event| event.name()).collect();
        assert_eq!(
            names,
            vec!["provider://refresh-started", "provider://refresh-completed"]
        );
    }
}
