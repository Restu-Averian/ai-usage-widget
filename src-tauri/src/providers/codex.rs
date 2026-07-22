use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::Mutex;
use tokio::time;

use crate::domain::{
    AppError, ConnectionType, ProviderCapabilities, ProviderId, ProviderUsage, Reliability,
    UsagePeriod, UsageWarning, UsageWindow,
};
use crate::process::{ProcessRequest, ProcessRunner, TokioProcessRunner};
use crate::providers::{FetchContext, ProviderConnector};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(20);
const MAX_LINE_BYTES: usize = 1024 * 1024;

#[derive(Clone)]
pub struct CodexProvider {
    executable: Option<PathBuf>,
    process_runner: Arc<dyn ProcessRunner>,
    rpc: Arc<dyn CodexRpc>,
}

impl CodexProvider {
    pub fn production() -> Self {
        let executable = find_codex_executable();
        let rpc: Arc<dyn CodexRpc> = Arc::new(ManagedAppServer::new(executable.clone()));
        Self {
            executable,
            process_runner: Arc::new(TokioProcessRunner),
            rpc,
        }
    }

    #[cfg(test)]
    fn test(
        executable: Option<PathBuf>,
        process_runner: Arc<dyn ProcessRunner>,
        rpc: Arc<dyn CodexRpc>,
    ) -> Self {
        Self {
            executable,
            process_runner,
            rpc,
        }
    }

    async fn validate_cli(&self) -> Result<String, AppError> {
        let executable = self
            .executable
            .clone()
            .ok_or_else(|| AppError::ProviderUnavailable("Codex CLI unavailable.".into()))?;
        let output = self
            .process_runner
            .run(ProcessRequest {
                executable,
                arguments: vec![OsString::from("--version")],
                environment: HashMap::new(),
                timeout: Duration::from_secs(3),
                max_output_bytes: 256,
                requires_tty: false,
            })
            .await
            .map_err(|_| AppError::ProviderUnavailable("Codex CLI unavailable.".into()))?;

        if !output.success {
            return Err(AppError::ProviderUnavailable(
                "Codex CLI unavailable.".into(),
            ));
        }
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !is_supported_version(&version) {
            return Err(AppError::Unsupported(format!(
                "Codex CLI version {version} is unsupported. Supported range is 0.144.x."
            )));
        }
        Ok(version)
    }
}

#[async_trait]
impl ProviderConnector for CodexProvider {
    fn id(&self) -> ProviderId {
        ProviderId::Codex
    }

    fn label(&self) -> &'static str {
        "Codex"
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            can_detect_installation: true,
            can_detect_authentication: true,
            can_start_login: true,
            can_fetch_subscription_usage: true,
            can_fetch_api_usage: false,
            can_disconnect_local_connection: false,
            requires_tty: false,
            supports_multiple_windows: true,
            supports_model_windows: false,
        }
    }

    async fn fetch_usage(&self, _context: FetchContext) -> Result<ProviderUsage, AppError> {
        let _version = self.validate_cli().await?;
        let account = self.rpc.account_read().await?;
        let account_meta = normalize_account(&account)?;
        let limits = self.rpc.rate_limits_read().await?;
        normalize_rate_limits(account_meta, limits)
    }

    async fn start_login(&self) -> Result<crate::domain::LoginLaunchResult, AppError> {
        let _version = self.validate_cli().await?;
        let login = self.rpc.login_start().await?;
        Ok(crate::domain::LoginLaunchResult {
            launched: true,
            message: Some(match login {
                LoginStartResponse::ChatGpt { auth_url, .. } => {
                    format!("Open this official Codex login URL: {auth_url}")
                }
                LoginStartResponse::ChatGptDeviceCode {
                    verification_url,
                    user_code,
                    ..
                } => format!("Open {verification_url} and enter code {user_code}."),
                LoginStartResponse::ApiKey => "Codex accepted API key login.".into(),
                LoginStartResponse::Other => "Codex login started.".into(),
            }),
        })
    }

    async fn shutdown(&self) {
        self.rpc.shutdown().await;
    }
}

#[async_trait]
pub trait CodexRpc: Send + Sync {
    async fn account_read(&self) -> Result<GetAccountResponse, AppError>;
    async fn rate_limits_read(&self) -> Result<GetAccountRateLimitsResponse, AppError>;
    async fn login_start(&self) -> Result<LoginStartResponse, AppError>;
    async fn shutdown(&self) {}
}

struct ManagedAppServer {
    executable: Option<PathBuf>,
    state: Mutex<Option<AppServerState>>,
}

struct AppServerState {
    child: Child,
    stdin: ChildStdin,
    lines: Lines<BufReader<ChildStdout>>,
    next_id: u64,
}

impl Drop for AppServerState {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}

impl ManagedAppServer {
    fn new(executable: Option<PathBuf>) -> Self {
        Self {
            executable,
            state: Mutex::new(None),
        }
    }

    async fn request(&self, method: &str, params: Value) -> Result<Value, AppError> {
        let mut guard = self.state.lock().await;
        if guard.is_none() {
            *guard = Some(
                start_app_server(self.executable.clone().ok_or_else(|| {
                    AppError::ProviderUnavailable("Codex CLI unavailable.".into())
                })?)
                .await?,
            );
        }

        let state = guard.as_mut().ok_or(AppError::Internal)?;
        if state
            .child
            .try_wait()
            .map_err(|_| AppError::Internal)?
            .is_some()
        {
            *guard = Some(
                start_app_server(self.executable.clone().ok_or_else(|| {
                    AppError::ProviderUnavailable("Codex CLI unavailable.".into())
                })?)
                .await?,
            );
        }
        let state = guard.as_mut().ok_or(AppError::Internal)?;
        let id = state.next_id;
        state.next_id += 1;
        let message = json!({ "method": method, "id": id, "params": params });
        state
            .stdin
            .write_all(format!("{message}\n").as_bytes())
            .await
            .map_err(|_| AppError::ProviderUnavailable("Codex app-server unavailable.".into()))?;
        let response = read_response(state, id).await;
        if response.is_err() {
            *guard = None;
        }
        response
    }

    async fn shutdown(&self) {
        if let Some(mut state) = self.state.lock().await.take() {
            let _ = state.child.start_kill();
            let _ = time::timeout(Duration::from_secs(2), state.child.wait()).await;
        }
    }
}

#[async_trait]
impl CodexRpc for ManagedAppServer {
    async fn account_read(&self) -> Result<GetAccountResponse, AppError> {
        let value = self.request("account/read", json!({})).await?;
        serde_json::from_value(value).map_err(|_| AppError::ContractViolation)
    }

    async fn rate_limits_read(&self) -> Result<GetAccountRateLimitsResponse, AppError> {
        let value = self.request("account/rateLimits/read", json!({})).await?;
        serde_json::from_value(value).map_err(|_| AppError::ContractViolation)
    }

    async fn login_start(&self) -> Result<LoginStartResponse, AppError> {
        let value = self
            .request(
                "account/login/start",
                json!({
                    "type": "chatgpt",
                    "appBrand": "codex",
                    "codexStreamlinedLogin": true,
                    "useHostedLoginSuccessPage": true
                }),
            )
            .await?;
        serde_json::from_value(value).map_err(|_| AppError::ContractViolation)
    }

    async fn shutdown(&self) {
        ManagedAppServer::shutdown(self).await;
    }
}

async fn start_app_server(executable: PathBuf) -> Result<AppServerState, AppError> {
    let mut child = Command::new(executable)
        .arg("app-server")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| AppError::ProviderUnavailable("Codex app-server failed to start.".into()))?;

    let stdin = child.stdin.take().ok_or(AppError::Internal)?;
    let stdout = child.stdout.take().ok_or(AppError::Internal)?;
    if let Some(stderr) = child.stderr.take() {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                let _ = redact_secret_like_text(&line);
                line.clear();
            }
        });
    }

    let lines = BufReader::new(stdout).lines();
    let mut state = AppServerState {
        child,
        stdin,
        lines,
        next_id: 2,
    };
    state
        .stdin
        .write_all(
            br#"{"method":"initialize","id":1,"params":{"clientInfo":{"name":"ai_usage_dock","title":"AI Usage Widget","version":"0.1.0"},"capabilities":{"experimentalApi":true}}}"#,
        )
        .await
        .map_err(|_| AppError::ProviderUnavailable("Codex app-server unavailable.".into()))?;
    state
        .stdin
        .write_all(b"\n")
        .await
        .map_err(|_| AppError::Internal)?;
    read_response(&mut state, 1).await?;
    state
        .stdin
        .write_all(br#"{"method":"initialized","params":{}}"#)
        .await
        .map_err(|_| AppError::Internal)?;
    state
        .stdin
        .write_all(b"\n")
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(state)
}

async fn read_response(state: &mut AppServerState, id: u64) -> Result<Value, AppError> {
    time::timeout(REQUEST_TIMEOUT, async {
        loop {
            let line = state
                .lines
                .next_line()
                .await
                .map_err(|_| {
                    AppError::ProviderUnavailable("Malformed Codex app-server output.".into())
                })?
                .ok_or_else(|| AppError::ProviderUnavailable("Codex app-server exited.".into()))?;
            let response = parse_response_line(&line, id)?;
            if let Some(response) = response {
                return Ok(response);
            }
        }
    })
    .await
    .map_err(|_| AppError::RetryableProviderError("Codex app-server request timed out.".into()))?
}

fn parse_response_line(line: &str, expected_id: u64) -> Result<Option<Value>, AppError> {
    if line.len() > MAX_LINE_BYTES {
        return Err(AppError::ContractViolation);
    }
    let value: Value = serde_json::from_str(line).map_err(|_| AppError::ContractViolation)?;
    if value.get("method").is_some() && value.get("id").is_none() {
        return Ok(None);
    }
    let Some(id) = value.get("id").and_then(Value::as_u64) else {
        return Err(AppError::ContractViolation);
    };
    if id != expected_id {
        return Err(AppError::ContractViolation);
    }
    if let Some(error) = value.get("error") {
        let message = error
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or("Codex app-server request failed.");
        return Err(map_rpc_error(message));
    }
    value
        .get("result")
        .cloned()
        .ok_or(AppError::ContractViolation)
        .map(Some)
}

fn map_rpc_error(message: &str) -> AppError {
    let lower = message.to_lowercase();
    if lower.contains("auth") || lower.contains("login") {
        AppError::AuthenticationExpired
    } else {
        AppError::RetryableProviderError("Codex app-server request failed.".into())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountResponse {
    pub account: Option<CodexAccount>,
    pub requires_openai_auth: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum CodexAccount {
    #[serde(rename = "apiKey")]
    ApiKey,
    #[serde(rename = "chatgpt")]
    ChatGpt {
        email: Option<String>,
        #[serde(rename = "planType")]
        plan_type: Option<String>,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum LoginStartResponse {
    #[serde(rename = "apiKey")]
    ApiKey,
    #[serde(rename = "chatgpt")]
    ChatGpt {
        #[serde(rename = "authUrl")]
        auth_url: String,
        #[serde(rename = "loginId")]
        login_id: String,
    },
    #[serde(rename = "chatgptDeviceCode")]
    ChatGptDeviceCode {
        #[serde(rename = "verificationUrl")]
        verification_url: String,
        #[serde(rename = "userCode")]
        user_code: String,
        #[serde(rename = "loginId")]
        login_id: String,
    },
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountRateLimitsResponse {
    pub rate_limits: RateLimitSnapshot,
    pub rate_limits_by_limit_id: Option<HashMap<String, RateLimitSnapshot>>,
    pub rate_limit_reset_credits: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitSnapshot {
    pub limit_id: Option<String>,
    pub limit_name: Option<String>,
    pub plan_type: Option<String>,
    pub primary: Option<RateLimitWindow>,
    pub secondary: Option<RateLimitWindow>,
    pub rate_limit_reached_type: Option<String>,
    pub credits: Option<Value>,
    pub individual_limit: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitWindow {
    pub used_percent: Option<i64>,
    pub window_duration_mins: Option<i64>,
    pub resets_at: Option<i64>,
}

struct AccountMeta {
    label: Option<String>,
    plan_name: Option<String>,
}

fn normalize_account(account: &GetAccountResponse) -> Result<AccountMeta, AppError> {
    if account.account.is_none() || !account.requires_openai_auth {
        return Err(AppError::AuthenticationExpired);
    }
    Ok(match &account.account {
        Some(CodexAccount::ChatGpt { email, plan_type }) => AccountMeta {
            label: email.clone(),
            plan_name: plan_type.clone(),
        },
        Some(CodexAccount::ApiKey) => AccountMeta {
            label: Some("Codex API key".into()),
            plan_name: Some("API key".into()),
        },
        _ => AccountMeta {
            label: None,
            plan_name: Some("Unknown plan".into()),
        },
    })
}

fn normalize_rate_limits(
    account: AccountMeta,
    response: GetAccountRateLimitsResponse,
) -> Result<ProviderUsage, AppError> {
    let snapshot = response
        .rate_limits_by_limit_id
        .as_ref()
        .and_then(|limits| limits.get("codex"))
        .cloned()
        .unwrap_or(response.rate_limits);
    let mut windows = Vec::new();
    if let Some(primary) = snapshot.primary {
        windows.push(normalize_window("primary", "Primary quota", primary)?);
    }
    if let Some(secondary) = snapshot.secondary {
        windows.push(normalize_window("secondary", "Secondary quota", secondary)?);
    }
    if windows.iter().all(|window| window.used_percent.is_none()) {
        windows
            .iter_mut()
            .for_each(|window| window.used_percent = None);
    }

    let mut usage = ProviderUsage::new(
        ProviderId::Codex,
        ConnectionType::SubscriptionCli,
        Reliability::OfficialCliJson,
        windows,
        Utc::now(),
    );
    usage.account_label = account.label;
    usage.plan_name = account.plan_name.or(snapshot.plan_type);
    if usage
        .windows
        .iter()
        .any(|window| window.used_percent.is_none())
    {
        usage.warnings.push(UsageWarning::PercentageUnavailable);
    }
    if snapshot.rate_limit_reached_type.is_some() {
        usage.warnings.push(UsageWarning::ResetTimePassed);
    }
    Ok(usage)
}

fn normalize_window(
    id: &str,
    fallback_label: &str,
    window: RateLimitWindow,
) -> Result<UsageWindow, AppError> {
    let label = match window.window_duration_mins {
        Some(300) => "5-hour quota".to_string(),
        Some(10080) => "Weekly quota".to_string(),
        Some(minutes) => format!("{minutes}-minute quota"),
        None => fallback_label.to_string(),
    };
    let period = match window.window_duration_mins {
        Some(300) => UsagePeriod::FiveHour,
        Some(10080) => UsagePeriod::Weekly,
        _ => UsagePeriod::Unknown,
    };
    let used = window.used_percent.map(|value| value as f64);
    let mut normalized = UsageWindow::new(id, label, period)
        .map_err(|_| AppError::ContractViolation)?
        .with_percentages(used, None)
        .map_err(|_| AppError::ContractViolation)?;
    if let Some(reset_at) = window.resets_at.and_then(unix_seconds_to_utc) {
        normalized = normalized.with_reset_at(reset_at);
    }
    Ok(normalized)
}

fn unix_seconds_to_utc(value: i64) -> Option<DateTime<Utc>> {
    Utc.timestamp_opt(value, 0).single()
}

fn find_codex_executable() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path) {
            let candidate = dir.join(executable_name("codex"));
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

fn executable_name(name: &str) -> String {
    if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}

fn is_supported_version(version: &str) -> bool {
    version
        .strip_prefix("codex-cli ")
        .and_then(|value| {
            value
                .split('.')
                .take(2)
                .collect::<Vec<_>>()
                .get(..2)
                .map(|v| v.to_vec())
        })
        .is_some_and(|parts| parts == ["0", "144"])
}

pub fn redact_secret_like_text(input: &str) -> String {
    input
        .split_whitespace()
        .map(|part| {
            if part.contains("sk-") || part.to_ascii_lowercase().contains("token") {
                "[REDACTED]"
            } else {
                part
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::{ProcessError, ProcessOutput};

    struct FakeRunner {
        stdout: Vec<u8>,
        success: bool,
    }

    #[async_trait]
    impl ProcessRunner for FakeRunner {
        async fn run(&self, _request: ProcessRequest) -> Result<ProcessOutput, ProcessError> {
            Ok(ProcessOutput {
                stdout: self.stdout.clone(),
                stderr: Vec::new(),
                stdout_truncated: false,
                stderr_truncated: false,
                exit_code: Some(if self.success { 0 } else { 1 }),
                success: self.success,
            })
        }
    }

    struct FakeRpc;

    #[async_trait]
    impl CodexRpc for FakeRpc {
        async fn account_read(&self) -> Result<GetAccountResponse, AppError> {
            Ok(sample_account())
        }

        async fn rate_limits_read(&self) -> Result<GetAccountRateLimitsResponse, AppError> {
            Ok(sample_limits(Some(28), None))
        }

        async fn login_start(&self) -> Result<LoginStartResponse, AppError> {
            Ok(LoginStartResponse::ChatGpt {
                auth_url: "https://chatgpt.com/".into(),
                login_id: "login_1".into(),
            })
        }

        async fn shutdown(&self) {}
    }

    #[test]
    fn parses_supported_and_unsupported_versions() {
        assert!(is_supported_version("codex-cli 0.144.5"));
        assert!(!is_supported_version("codex-cli 0.143.9"));
    }

    #[tokio::test]
    async fn executable_not_found_returns_provider_unavailable() {
        let provider = CodexProvider::test(
            None,
            Arc::new(FakeRunner {
                stdout: b"codex-cli 0.144.5".to_vec(),
                success: true,
            }),
            Arc::new(FakeRpc),
        );

        assert!(matches!(
            provider.fetch_usage(FetchContext::manual()).await,
            Err(AppError::ProviderUnavailable(_))
        ));
    }

    #[tokio::test]
    async fn unsupported_version_returns_unsupported() {
        let provider = CodexProvider::test(
            Some(PathBuf::from("/usr/local/bin/codex")),
            Arc::new(FakeRunner {
                stdout: b"codex-cli 0.143.0".to_vec(),
                success: true,
            }),
            Arc::new(FakeRpc),
        );

        assert!(matches!(
            provider.fetch_usage(FetchContext::manual()).await,
            Err(AppError::Unsupported(_))
        ));
    }

    #[test]
    fn maps_primary_and_secondary_windows_without_inverting_used_percentage() {
        let usage = normalize_rate_limits(
            AccountMeta {
                label: Some("user@example.test".into()),
                plan_name: Some("plus".into()),
            },
            sample_limits(Some(28), Some(75)),
        )
        .expect("usage");

        assert_eq!(usage.windows[0].used_percent, Some(28.0));
        assert_eq!(usage.windows[0].remaining_percent, Some(72.0));
        assert_eq!(usage.windows[1].used_percent, Some(75.0));
    }

    #[test]
    fn preserves_missing_percentage_and_missing_secondary_window() {
        let mut limits = sample_limits(None, None);
        limits.rate_limits.secondary = None;
        let usage = normalize_rate_limits(
            AccountMeta {
                label: None,
                plan_name: None,
            },
            limits,
        )
        .expect("usage");

        assert_eq!(usage.windows.len(), 1);
        assert_eq!(usage.windows[0].used_percent, None);
        assert!(usage
            .warnings
            .contains(&UsageWarning::PercentageUnavailable));
    }

    #[test]
    fn rejects_malformed_and_mismatched_json_rpc_responses() {
        assert!(parse_response_line("not-json", 1).is_err());
        assert!(parse_response_line(r#"{"id":2,"result":{}}"#, 1).is_err());
        assert_eq!(
            parse_response_line(r#"{"method":"account/updated","params":{}}"#, 1)
                .expect("notification"),
            None
        );
    }

    #[test]
    fn redacts_secret_like_log_text() {
        let redacted = redact_secret_like_text("token abc sk-secret authorization");
        assert!(!redacted.contains("sk-secret"));
        assert!(redacted.contains("[REDACTED]"));
    }

    #[tokio::test]
    async fn account_read_authenticated_and_unauthenticated_mapping() {
        let authenticated = normalize_account(&sample_account()).expect("authenticated");
        assert_eq!(authenticated.plan_name, Some("plus".into()));

        let unauthenticated = GetAccountResponse {
            account: None,
            requires_openai_auth: true,
        };
        assert!(matches!(
            normalize_account(&unauthenticated),
            Err(AppError::AuthenticationExpired)
        ));
    }

    fn sample_account() -> GetAccountResponse {
        GetAccountResponse {
            account: Some(CodexAccount::ChatGpt {
                email: Some("user@example.test".into()),
                plan_type: Some("plus".into()),
            }),
            requires_openai_auth: true,
        }
    }

    fn sample_limits(primary: Option<i64>, secondary: Option<i64>) -> GetAccountRateLimitsResponse {
        GetAccountRateLimitsResponse {
            rate_limits: RateLimitSnapshot {
                limit_id: Some("codex".into()),
                limit_name: Some("Codex".into()),
                plan_type: Some("plus".into()),
                primary: Some(RateLimitWindow {
                    used_percent: primary,
                    window_duration_mins: Some(10080),
                    resets_at: Some(1784949864),
                }),
                secondary: secondary.map(|used_percent| RateLimitWindow {
                    used_percent: Some(used_percent),
                    window_duration_mins: Some(300),
                    resets_at: Some(1784350000),
                }),
                rate_limit_reached_type: None,
                credits: None,
                individual_limit: None,
            },
            rate_limits_by_limit_id: None,
            rate_limit_reset_credits: None,
        }
    }
}
