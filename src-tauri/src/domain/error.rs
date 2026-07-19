use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppErrorPayload {
    pub code: AppErrorCode,
    pub message: String,
    pub retryable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppErrorCode {
    Internal,
    InvalidInput,
    ProviderUnavailable,
    AuthenticationExpired,
    NetworkUnavailable,
    RetryableProviderError,
    Unsupported,
    Database,
    SecretStore,
    ContractViolation,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("provider unavailable: {0}")]
    ProviderUnavailable(String),
    #[error("authentication expired")]
    AuthenticationExpired,
    #[error("network unavailable")]
    NetworkUnavailable,
    #[error("provider refresh failed: {0}")]
    RetryableProviderError(String),
    #[error("unsupported provider capability: {0}")]
    Unsupported(String),
    #[error("database operation failed")]
    Database,
    #[error("secret store operation failed")]
    SecretStore,
    #[error("frontend/backend contract violation")]
    ContractViolation,
    #[error("internal application error")]
    Internal,
}

impl AppError {
    pub fn payload(&self) -> AppErrorPayload {
        match self {
            AppError::InvalidInput(message) => AppErrorPayload {
                code: AppErrorCode::InvalidInput,
                message: message.clone(),
                retryable: false,
            },
            AppError::ProviderUnavailable(message) => AppErrorPayload {
                code: AppErrorCode::ProviderUnavailable,
                message: message.clone(),
                retryable: true,
            },
            AppError::AuthenticationExpired => AppErrorPayload {
                code: AppErrorCode::AuthenticationExpired,
                message: "Authentication expired. Reconnect the provider.".into(),
                retryable: false,
            },
            AppError::NetworkUnavailable => AppErrorPayload {
                code: AppErrorCode::NetworkUnavailable,
                message: "Network unavailable. Showing saved data when available.".into(),
                retryable: true,
            },
            AppError::RetryableProviderError(message) => AppErrorPayload {
                code: AppErrorCode::RetryableProviderError,
                message: message.clone(),
                retryable: true,
            },
            AppError::Unsupported(message) => AppErrorPayload {
                code: AppErrorCode::Unsupported,
                message: message.clone(),
                retryable: false,
            },
            AppError::Database => AppErrorPayload {
                code: AppErrorCode::Database,
                message: "Local database operation failed.".into(),
                retryable: false,
            },
            AppError::SecretStore => AppErrorPayload {
                code: AppErrorCode::SecretStore,
                message: "Credential storage operation failed.".into(),
                retryable: false,
            },
            AppError::ContractViolation => AppErrorPayload {
                code: AppErrorCode::ContractViolation,
                message: "Internal data contract mismatch.".into(),
                retryable: false,
            },
            AppError::Internal => AppErrorPayload {
                code: AppErrorCode::Internal,
                message: "Internal application error.".into(),
                retryable: false,
            },
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum DomainError {
    #[error("{field} percentage must be between 0 and 100, got {value}")]
    InvalidPercentage { field: &'static str, value: f64 },
    #[error("{0} must not be empty")]
    EmptyField(&'static str),
}
