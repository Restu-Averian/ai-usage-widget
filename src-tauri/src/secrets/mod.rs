use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use secrecy::SecretString;
use tokio::sync::Mutex;

use crate::domain::{AppError, ConnectionType, ProviderId};

const SERVICE_NAME: &str = "ai-usage-dock";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecretKey {
    provider: ProviderId,
    connection_type: ConnectionType,
}

impl SecretKey {
    pub fn new(provider: ProviderId, connection_type: ConnectionType) -> Self {
        Self {
            provider,
            connection_type,
        }
    }

    fn account(&self) -> String {
        format!("{}:{}", self.provider, self.connection_type)
    }
}

#[async_trait]
pub trait SecretStore: Send + Sync {
    async fn set(&self, key: &SecretKey, value: &str) -> Result<(), AppError>;
    async fn exists(&self, key: &SecretKey) -> Result<bool, AppError>;
    async fn consume(&self, key: &SecretKey) -> Result<Option<SecretString>, AppError>;
    async fn delete(&self, key: &SecretKey) -> Result<(), AppError>;
}

#[derive(Default, Clone)]
pub struct MemorySecretStore {
    values: Arc<Mutex<HashMap<SecretKey, SecretString>>>,
}

#[async_trait]
impl SecretStore for MemorySecretStore {
    async fn set(&self, key: &SecretKey, value: &str) -> Result<(), AppError> {
        self.values
            .lock()
            .await
            .insert(key.clone(), SecretString::from(value.to_string()));
        Ok(())
    }

    async fn exists(&self, key: &SecretKey) -> Result<bool, AppError> {
        Ok(self.values.lock().await.contains_key(key))
    }

    async fn consume(&self, key: &SecretKey) -> Result<Option<SecretString>, AppError> {
        Ok(self.values.lock().await.get(key).cloned())
    }

    async fn delete(&self, key: &SecretKey) -> Result<(), AppError> {
        self.values.lock().await.remove(key);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NativeSecretStore;

#[async_trait]
impl SecretStore for NativeSecretStore {
    async fn set(&self, key: &SecretKey, value: &str) -> Result<(), AppError> {
        let account = key.account();
        let secret = value.to_string();
        tokio::task::spawn_blocking(move || {
            keyring::Entry::new(SERVICE_NAME, &account)
                .map_err(|_| AppError::SecretStore)?
                .set_password(&secret)
                .map_err(|_| AppError::SecretStore)
        })
        .await
        .map_err(|_| AppError::SecretStore)?
    }

    async fn exists(&self, key: &SecretKey) -> Result<bool, AppError> {
        Ok(self.consume(key).await?.is_some())
    }

    async fn consume(&self, key: &SecretKey) -> Result<Option<SecretString>, AppError> {
        let account = key.account();
        tokio::task::spawn_blocking(move || {
            match keyring::Entry::new(SERVICE_NAME, &account)
                .map_err(|_| AppError::SecretStore)?
                .get_password()
            {
                Ok(value) => Ok(Some(SecretString::from(value))),
                Err(keyring::Error::NoEntry) => Ok(None),
                Err(_) => Err(AppError::SecretStore),
            }
        })
        .await
        .map_err(|_| AppError::SecretStore)?
    }

    async fn delete(&self, key: &SecretKey) -> Result<(), AppError> {
        let account = key.account();
        tokio::task::spawn_blocking(move || {
            match keyring::Entry::new(SERVICE_NAME, &account)
                .map_err(|_| AppError::SecretStore)?
                .delete_credential()
            {
                Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
                Err(_) => Err(AppError::SecretStore),
            }
        })
        .await
        .map_err(|_| AppError::SecretStore)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ConnectionType, ProviderId};
    use secrecy::ExposeSecret;

    #[tokio::test]
    async fn memory_secret_store_sets_consumes_and_deletes_secret() {
        let store = MemorySecretStore::default();
        let key = SecretKey::new(ProviderId::Codex, ConnectionType::AdminApiKey);

        store.set(&key, "sk-test-secret").await.expect("set");
        assert!(store.exists(&key).await.expect("exists"));
        assert_eq!(
            store
                .consume(&key)
                .await
                .expect("consume")
                .as_ref()
                .map(|secret| secret.expose_secret()),
            Some("sk-test-secret")
        );

        store.delete(&key).await.expect("delete");
        assert!(!store.exists(&key).await.expect("missing"));
    }
}
