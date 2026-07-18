use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::domain::{
    AppErrorPayload, AppSettings, ConnectionType, HistoryPoint, ProviderId, ProviderMetadata,
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
pub struct HistoryRequest {
    pub provider: ProviderId,
    pub limit: Option<i64>,
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
        Err(error) => CommandResult::err(error.payload()),
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

pub async fn get_usage_history_data(
    state: &AppState,
    request: HistoryRequest,
) -> CommandResult<Vec<HistoryPoint>> {
    match state
        .database
        .history(request.provider, request.limit.unwrap_or(30))
        .await
    {
        Ok(history) => CommandResult::ok(history),
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

#[tauri::command]
pub async fn get_app_bootstrap(
    state: tauri::State<'_, AppState>,
) -> Result<CommandResult<AppBootstrap>, String> {
    Ok(get_app_bootstrap_data(&state).await)
}

#[tauri::command]
pub async fn list_providers(
    state: tauri::State<'_, AppState>,
) -> Result<CommandResult<Vec<ProviderMetadata>>, String> {
    Ok(list_providers_data(&state).await)
}

#[tauri::command]
pub async fn get_provider_state(
    state: tauri::State<'_, AppState>,
    request: ProviderRequest,
) -> Result<CommandResult<ProviderState>, String> {
    Ok(get_provider_state_data(&state, request.provider).await)
}

#[tauri::command]
pub async fn refresh_provider(
    state: tauri::State<'_, AppState>,
    request: ProviderRequest,
) -> Result<CommandResult<ProviderState>, String> {
    if let Some(scenario) = request.scenario {
        let _ = state
            .providers
            .set_fake_scenario(request.provider, scenario)
            .await;
    }
    Ok(refresh_provider_data(&state, request.provider).await)
}

#[tauri::command]
pub async fn refresh_all_providers(
    state: tauri::State<'_, AppState>,
) -> Result<CommandResult<Vec<ProviderState>>, String> {
    Ok(refresh_all_providers_data(&state).await)
}

#[tauri::command]
pub async fn get_settings(
    state: tauri::State<'_, AppState>,
) -> Result<CommandResult<AppSettings>, String> {
    Ok(get_settings_data(&state).await)
}

#[tauri::command]
pub async fn update_settings(
    state: tauri::State<'_, AppState>,
    settings: AppSettings,
) -> Result<CommandResult<AppSettings>, String> {
    Ok(update_settings_data(&state, settings).await)
}

#[tauri::command]
pub async fn get_usage_history(
    state: tauri::State<'_, AppState>,
    request: HistoryRequest,
) -> Result<CommandResult<Vec<HistoryPoint>>, String> {
    Ok(get_usage_history_data(&state, request).await)
}

#[tauri::command]
pub async fn save_provider_api_key(
    state: tauri::State<'_, AppState>,
    request: SaveProviderSecretRequest,
) -> Result<CommandResult<SecretMutationResult>, String> {
    Ok(save_provider_secret_data(&state, request).await)
}

#[tauri::command]
pub async fn delete_provider_api_key(
    state: tauri::State<'_, AppState>,
    request: DeleteProviderSecretRequest,
) -> Result<CommandResult<SecretMutationResult>, String> {
    Ok(delete_provider_secret_data(&state, request).await)
}

#[tauri::command]
pub async fn set_fake_provider_scenario(
    state: tauri::State<'_, AppState>,
    request: FakeScenarioRequest,
) -> Result<CommandResult<ProviderState>, String> {
    Ok(set_fake_provider_scenario_data(&state, request).await)
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
        assert_eq!(result.data.expect("data").providers.len(), 3);
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
}
