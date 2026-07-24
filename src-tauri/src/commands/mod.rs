use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::app_state::{AppState, AppStateHandle};
use crate::domain::{
    AppErrorPayload, AppSettings, ConnectionType, LoginLaunchResult, ProviderId, ProviderMetadata,
    ProviderStateKind, ProviderUsage,
};
use crate::providers::FakeProviderScenario;
use crate::secrets::SecretKey;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandResult<T>
where
    T: Serialize,
{
    pub ok: bool,
    pub data: Option<T>,
    pub error: Option<AppErrorPayload>,
    pub request_id: String,
}

impl<T> CommandResult<T>
where
    T: Serialize,
{
    fn ok(data: T) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
            request_id: Uuid::new_v4().to_string(),
        }
    }

    fn err(error: AppErrorPayload) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(error),
            request_id: Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBootstrap {
    pub providers: Vec<ProviderMetadata>,
    pub provider_states: Vec<ProviderState>,
    pub settings: AppSettings,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderState {
    pub provider: ProviderId,
    pub status: ProviderStateKind,
    pub usage: Option<ProviderUsage>,
    pub last_error: Option<AppErrorPayload>,
    pub is_refreshing: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRequest {
    pub provider: ProviderId,
    pub scenario: Option<FakeProviderScenario>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveProviderSecretRequest {
    pub provider: ProviderId,
    pub connection_type: ConnectionType,
    pub secret: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProviderSecretRequest {
    pub provider: ProviderId,
    pub connection_type: ConnectionType,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FakeScenarioRequest {
    pub provider: ProviderId,
    pub scenario: FakeProviderScenario,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretMutationResult {
    pub exists: bool,
}

pub async fn get_app_bootstrap_data(state: &AppState) -> CommandResult<AppBootstrap> {
    let settings = match state.database.load_settings().await {
        Ok(settings) => settings,
        Err(error) => return CommandResult::err(error.payload()),
    };
    let mut provider_states = Vec::new();
    for provider in ProviderId::ALL {
        match provider_state_data(state, provider).await {
            Ok(provider_state) => provider_states.push(provider_state),
            Err(error) => return CommandResult::err(error),
        }
    }

    CommandResult::ok(AppBootstrap {
        providers: state.providers.list_metadata(),
        provider_states,
        settings,
    })
}

pub async fn list_providers_data(state: &AppState) -> CommandResult<Vec<ProviderMetadata>> {
    CommandResult::ok(state.providers.list_metadata())
}

pub async fn get_provider_state_data(
    state: &AppState,
    provider: ProviderId,
) -> CommandResult<ProviderState> {
    match provider_state_data(state, provider).await {
        Ok(provider_state) => CommandResult::ok(provider_state),
        Err(error) => CommandResult::err(error),
    }
}

pub async fn refresh_provider_data(
    state: &AppState,
    provider: ProviderId,
) -> CommandResult<ProviderState> {
    match state.scheduler.refresh_provider(state, provider).await {
        Ok(provider_state) => CommandResult::ok(provider_state),
        Err(error) => {
            let payload = error.payload();
            CommandResult::ok(ProviderState {
                provider,
                status: status_from_error(&payload.code),
                usage: None,
                last_error: Some(payload),
                is_refreshing: false,
            })
        }
    }
}

pub async fn set_fake_provider_scenario_data(
    state: &AppState,
    request: FakeScenarioRequest,
) -> CommandResult<ProviderState> {
    if let Err(error) = state
        .providers
        .set_fake_scenario(request.provider, request.scenario)
        .await
    {
        return CommandResult::err(error.payload());
    }
    refresh_provider_data(state, request.provider).await
}

pub async fn refresh_all_providers_data(state: &AppState) -> CommandResult<Vec<ProviderState>> {
    let mut states = Vec::new();
    for provider in ProviderId::ALL {
        let result = refresh_provider_data(state, provider).await;
        if let Some(provider_state) = result.data {
            states.push(provider_state);
        }
    }
    CommandResult::ok(states)
}

pub async fn start_provider_login_data(
    state: &AppState,
    provider: ProviderId,
) -> CommandResult<LoginLaunchResult> {
    match state.providers.start_login(provider).await {
        Ok(result) => CommandResult::ok(result),
        Err(error) => CommandResult::err(error.payload()),
    }
}

pub async fn get_settings_data(state: &AppState) -> CommandResult<AppSettings> {
    match state.database.load_settings().await {
        Ok(settings) => CommandResult::ok(settings),
        Err(error) => CommandResult::err(error.payload()),
    }
}

pub async fn update_settings_data(
    state: &AppState,
    settings: AppSettings,
) -> CommandResult<AppSettings> {
    match state.database.save_settings(&settings).await {
        Ok(()) => CommandResult::ok(settings),
        Err(error) => CommandResult::err(error.payload()),
    }
}

pub async fn save_provider_secret_data(
    state: &AppState,
    request: SaveProviderSecretRequest,
) -> CommandResult<SecretMutationResult> {
    let key = SecretKey::new(request.provider, request.connection_type);
    match state.secrets.set(&key, &request.secret).await {
        Ok(()) => CommandResult::ok(SecretMutationResult { exists: true }),
        Err(error) => CommandResult::err(error.payload()),
    }
}

pub async fn delete_provider_secret_data(
    state: &AppState,
    request: DeleteProviderSecretRequest,
) -> CommandResult<SecretMutationResult> {
    let key = SecretKey::new(request.provider, request.connection_type);
    match state.secrets.delete(&key).await {
        Ok(()) => CommandResult::ok(SecretMutationResult { exists: false }),
        Err(error) => CommandResult::err(error.payload()),
    }
}

async fn provider_state_data(
    state: &AppState,
    provider: ProviderId,
) -> Result<ProviderState, AppErrorPayload> {
    state
        .database
        .latest_usage(provider)
        .await
        .map(|usage| ProviderState {
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
        .map_err(|error| error.payload())
}

fn status_from_error(code: &crate::domain::AppErrorCode) -> ProviderStateKind {
    match code {
        crate::domain::AppErrorCode::ProviderUnavailable => ProviderStateKind::NotInstalled,
        crate::domain::AppErrorCode::AuthenticationExpired => {
            ProviderStateKind::AuthenticationRequired
        }
        crate::domain::AppErrorCode::Unsupported => ProviderStateKind::Unsupported,
        crate::domain::AppErrorCode::NetworkUnavailable => ProviderStateKind::Offline,
        _ => ProviderStateKind::Error,
    }
}

#[tauri::command]
pub async fn get_app_bootstrap(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<AppStateHandle>>,
) -> Result<CommandResult<AppBootstrap>, String> {
    let state = match state.get().await {
        Ok(state) => state,
        Err(error) => return Ok(CommandResult::err(error)),
    };
    let result = get_app_bootstrap_data(&state).await;
    if let Some(data) = &result.data {
        if let Some(codex) = data
            .provider_states
            .iter()
            .find(|provider| provider.provider == ProviderId::Codex)
        {
            crate::tray::update_codex_menu_item(&app, codex);
        }
    }
    Ok(result)
}

#[tauri::command]
pub async fn list_providers(
    state: tauri::State<'_, Arc<AppStateHandle>>,
) -> Result<CommandResult<Vec<ProviderMetadata>>, String> {
    let state = match state.get().await {
        Ok(state) => state,
        Err(error) => return Ok(CommandResult::err(error)),
    };
    Ok(list_providers_data(&state).await)
}

#[tauri::command]
pub async fn get_provider_state(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<AppStateHandle>>,
    request: ProviderRequest,
) -> Result<CommandResult<ProviderState>, String> {
    let state = match state.get().await {
        Ok(state) => state,
        Err(error) => return Ok(CommandResult::err(error)),
    };
    let result = get_provider_state_data(&state, request.provider).await;
    if request.provider == ProviderId::Codex {
        if let Some(data) = &result.data {
            crate::tray::update_codex_menu_item(&app, data);
        }
    }
    Ok(result)
}

#[tauri::command]
pub async fn refresh_provider(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<AppStateHandle>>,
    request: ProviderRequest,
) -> Result<CommandResult<ProviderState>, String> {
    let state = match state.get().await {
        Ok(state) => state,
        Err(error) => return Ok(CommandResult::err(error)),
    };
    #[cfg(debug_assertions)]
    if let Some(scenario) = request.scenario {
        let _ = state
            .providers
            .set_fake_scenario(request.provider, scenario)
            .await;
    }
    let result = refresh_provider_data(&state, request.provider).await;
    if request.provider == ProviderId::Codex {
        if let Some(data) = &result.data {
            crate::tray::update_codex_menu_item(&app, data);
        }
    }
    Ok(result)
}

#[tauri::command]
pub async fn refresh_all_providers(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<AppStateHandle>>,
) -> Result<CommandResult<Vec<ProviderState>>, String> {
    let state = match state.get().await {
        Ok(state) => state,
        Err(error) => return Ok(CommandResult::err(error)),
    };
    let result = refresh_all_providers_data(&state).await;
    if let Some(states) = &result.data {
        if let Some(codex) = states
            .iter()
            .find(|provider| provider.provider == ProviderId::Codex)
        {
            crate::tray::update_codex_menu_item(&app, codex);
        }
    }
    Ok(result)
}

#[tauri::command]
pub async fn start_provider_login(
    state: tauri::State<'_, Arc<AppStateHandle>>,
    request: ProviderRequest,
) -> Result<CommandResult<LoginLaunchResult>, String> {
    let state = match state.get().await {
        Ok(state) => state,
        Err(error) => return Ok(CommandResult::err(error)),
    };
    Ok(start_provider_login_data(&state, request.provider).await)
}

#[tauri::command]
pub async fn get_settings(
    state: tauri::State<'_, Arc<AppStateHandle>>,
) -> Result<CommandResult<AppSettings>, String> {
    let state = match state.get().await {
        Ok(state) => state,
        Err(error) => return Ok(CommandResult::err(error)),
    };
    Ok(get_settings_data(&state).await)
}

#[tauri::command]
pub async fn update_settings(
    state: tauri::State<'_, Arc<AppStateHandle>>,
    settings: AppSettings,
) -> Result<CommandResult<AppSettings>, String> {
    let state = match state.get().await {
        Ok(state) => state,
        Err(error) => return Ok(CommandResult::err(error)),
    };
    Ok(update_settings_data(&state, settings).await)
}

#[tauri::command]
pub async fn save_provider_api_key(
    state: tauri::State<'_, Arc<AppStateHandle>>,
    request: SaveProviderSecretRequest,
) -> Result<CommandResult<SecretMutationResult>, String> {
    let state = match state.get().await {
        Ok(state) => state,
        Err(error) => return Ok(CommandResult::err(error)),
    };
    Ok(save_provider_secret_data(&state, request).await)
}

#[tauri::command]
pub async fn delete_provider_api_key(
    state: tauri::State<'_, Arc<AppStateHandle>>,
    request: DeleteProviderSecretRequest,
) -> Result<CommandResult<SecretMutationResult>, String> {
    let state = match state.get().await {
        Ok(state) => state,
        Err(error) => return Ok(CommandResult::err(error)),
    };
    Ok(delete_provider_secret_data(&state, request).await)
}

#[tauri::command]
pub async fn set_fake_provider_scenario(
    state: tauri::State<'_, Arc<AppStateHandle>>,
    request: FakeScenarioRequest,
) -> Result<CommandResult<ProviderState>, String> {
    #[cfg(not(debug_assertions))]
    {
        let _ = state;
        let _ = request;
        return Ok(CommandResult::err(
            crate::domain::AppError::InvalidInput(
                "Development mock scenarios are disabled in production.".into(),
            )
            .payload(),
        ));
    }

    #[cfg(debug_assertions)]
    {
        let state = match state.get().await {
            Ok(state) => state,
            Err(error) => return Ok(CommandResult::err(error)),
        };
        Ok(set_fake_provider_scenario_data(&state, request).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ConnectionType, ProviderId};

    #[tokio::test]
    async fn bootstrap_returns_success_envelope() {
        let state = crate::app_state::AppState::test().await.expect("state");
        let result = get_app_bootstrap_data(&state).await;

        assert!(result.ok);
        assert_eq!(result.data.expect("data").providers.len(), 2);
    }

    #[tokio::test]
    async fn save_secret_response_does_not_return_secret() {
        let state = crate::app_state::AppState::test().await.expect("state");
        let result = save_provider_secret_data(
            &state,
            SaveProviderSecretRequest {
                provider: ProviderId::Codex,
                connection_type: ConnectionType::AdminApiKey,
                secret: "sk-test-secret".to_string(),
            },
        )
        .await;

        assert!(result.ok);
        assert!(result.data.expect("data").exists);
    }

    #[tokio::test]
    async fn fake_scenario_command_flows_through_refresh_and_persistence() {
        let state = crate::app_state::AppState::test().await.expect("state");
        let result = set_fake_provider_scenario_data(
            &state,
            FakeScenarioRequest {
                provider: ProviderId::Codex,
                scenario: FakeProviderScenario::UnknownPercentage,
            },
        )
        .await;

        assert!(result.ok);
        assert_eq!(
            state
                .database
                .latest_usage(ProviderId::Codex)
                .await
                .expect("latest")
                .expect("usage")
                .windows[0]
                .used_percent,
            None
        );
    }

    #[tokio::test]
    async fn refresh_error_returns_typed_provider_state() {
        let state = crate::app_state::AppState::test_with_registry(
            crate::providers::ProviderRegistry::fake_with_scenario(
                crate::providers::FakeProviderScenario::RetryableError,
            ),
        )
        .await
        .expect("state");

        let result = refresh_provider_data(&state, ProviderId::Codex).await;
        let provider_state = result.data.expect("provider state");

        assert!(result.ok);
        assert_eq!(provider_state.status, ProviderStateKind::Error);
        assert!(provider_state.last_error.is_some());
    }
}
