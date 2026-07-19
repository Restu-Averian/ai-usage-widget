use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, Utc};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{Executor, Row, SqlitePool};
use uuid::Uuid;

use crate::domain::{
    AppError, AppSettings, ConnectionType, ProviderConnection, ProviderId, ProviderStateKind,
    ProviderUsage, Reliability, UsagePeriod, UsageWindow,
};

const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    name: "m4_core_schema",
    sql: r#"
CREATE TABLE IF NOT EXISTS provider_connections (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL,
    connection_type TEXT NOT NULL,
    account_label TEXT,
    plan_name TEXT,
    status TEXT NOT NULL,
    executable_path TEXT,
    cli_version TEXT,
    reliability TEXT,
    last_success_at TEXT,
    last_attempt_at TEXT,
    last_error_code TEXT,
    last_error_message TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(provider, connection_type)
);

CREATE TABLE IF NOT EXISTS usage_snapshots (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL,
    connection_type TEXT NOT NULL,
    account_label TEXT,
    plan_name TEXT,
    reliability TEXT NOT NULL,
    fetched_at TEXT NOT NULL,
    captured_at TEXT NOT NULL,
    stale INTEGER NOT NULL DEFAULT 0,
    warning_json TEXT NOT NULL DEFAULT '[]'
);

CREATE INDEX IF NOT EXISTS idx_usage_snapshots_provider_time
ON usage_snapshots(provider, captured_at DESC);

CREATE TABLE IF NOT EXISTS usage_windows (
    id TEXT PRIMARY KEY,
    snapshot_id TEXT NOT NULL,
    external_window_id TEXT NOT NULL,
    label TEXT NOT NULL,
    period TEXT NOT NULL,
    model TEXT,
    used_percent REAL,
    remaining_percent REAL,
    reset_at TEXT,
    derived_used_percent INTEGER NOT NULL DEFAULT 0,
    derived_remaining_percent INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(snapshot_id) REFERENCES usage_snapshots(id) ON DELETE CASCADE,
    CHECK(used_percent IS NULL OR (used_percent >= 0 AND used_percent <= 100)),
    CHECK(remaining_percent IS NULL OR (remaining_percent >= 0 AND remaining_percent <= 100))
);

CREATE INDEX IF NOT EXISTS idx_usage_windows_snapshot
ON usage_windows(snapshot_id);

CREATE TABLE IF NOT EXISTS api_usage_totals (
    id TEXT PRIMARY KEY,
    snapshot_id TEXT NOT NULL,
    input_tokens INTEGER,
    output_tokens INTEGER,
    cached_tokens INTEGER,
    request_count INTEGER,
    cost_used REAL,
    budget REAL,
    currency TEXT,
    FOREIGN KEY(snapshot_id) REFERENCES usage_snapshots(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS notification_states (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL,
    connection_type TEXT NOT NULL,
    window_key TEXT NOT NULL,
    threshold INTEGER NOT NULL,
    period_identifier TEXT NOT NULL,
    notified_at TEXT NOT NULL,
    UNIQUE(provider, connection_type, window_key, threshold, period_identifier)
);

CREATE TABLE IF NOT EXISTS provider_executables (
    provider TEXT PRIMARY KEY,
    executable_path TEXT NOT NULL,
    cli_version TEXT,
    approval_status TEXT NOT NULL,
    approved_at TEXT,
    last_seen_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS app_settings (
    key TEXT PRIMARY KEY,
    value_json TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
"#,
}];

struct Migration {
    version: i64,
    name: &'static str,
    sql: &'static str,
}

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect(path: impl AsRef<Path>) -> Result<Self, AppError> {
        let url = format!("sqlite://{}", path.as_ref().display());
        let options = SqliteConnectOptions::from_str(&url)
            .map_err(|_| AppError::Database)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .foreign_keys(true)
            .busy_timeout(Duration::from_secs(5));
        Self::connect_with_options(options).await
    }

    pub async fn in_memory() -> Result<Self, AppError> {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")
            .map_err(|_| AppError::Database)?
            .foreign_keys(true);
        Self::connect_with_options(options).await
    }

    async fn connect_with_options(options: SqliteConnectOptions) -> Result<Self, AppError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await
            .map_err(|_| AppError::Database)?;

        let database = Self { pool };
        database.run_migrations().await?;
        Ok(database)
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub async fn run_migrations(&self) -> Result<(), AppError> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&self.pool)
        .await
        .map_err(|_| AppError::Database)?;

        for migration in MIGRATIONS {
            let applied: Option<(i64,)> =
                sqlx::query_as("SELECT version FROM schema_migrations WHERE version = ?")
                    .bind(migration.version)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|_| AppError::Database)?;

            if applied.is_some() {
                continue;
            }

            let mut tx = self.pool.begin().await.map_err(|_| AppError::Database)?;
            tx.execute(migration.sql)
                .await
                .map_err(|_| AppError::Database)?;
            sqlx::query("INSERT INTO schema_migrations (version, name) VALUES (?, ?)")
                .bind(migration.version)
                .bind(migration.name)
                .execute(&mut *tx)
                .await
                .map_err(|_| AppError::Database)?;
            tx.commit().await.map_err(|_| AppError::Database)?;
        }

        Ok(())
    }

    pub async fn save_usage_snapshot(&self, usage: &ProviderUsage) -> Result<(), AppError> {
        if let Some(latest) = self.latest_usage(usage.provider).await? {
            if snapshots_match_meaningfully(&latest, usage) {
                return Ok(());
            }
        }

        let mut tx = self.pool.begin().await.map_err(|_| AppError::Database)?;
        sqlx::query("DELETE FROM usage_snapshots WHERE provider = ?")
            .bind(serde_string(&usage.provider)?)
            .execute(&mut *tx)
            .await
            .map_err(|_| AppError::Database)?;
        sqlx::query(
            "INSERT INTO usage_snapshots (
                id, provider, connection_type, account_label, plan_name, reliability,
                fetched_at, captured_at, stale, warning_json
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&usage.id)
        .bind(serde_string(&usage.provider)?)
        .bind(serde_string(&usage.connection_type)?)
        .bind(&usage.account_label)
        .bind(&usage.plan_name)
        .bind(serde_string(&usage.reliability)?)
        .bind(usage.fetched_at.to_rfc3339())
        .bind(usage.fetched_at.to_rfc3339())
        .bind(usage.stale)
        .bind(serde_json::to_string(&usage.warnings).map_err(|_| AppError::Database)?)
        .execute(&mut *tx)
        .await
        .map_err(|_| AppError::Database)?;

        for window in &usage.windows {
            sqlx::query(
                "INSERT INTO usage_windows (
                    id, snapshot_id, external_window_id, label, period, model,
                    used_percent, remaining_percent, reset_at,
                    derived_used_percent, derived_remaining_percent
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(Uuid::new_v4().to_string())
            .bind(&usage.id)
            .bind(&window.id)
            .bind(&window.label)
            .bind(serde_string(&window.period)?)
            .bind(&window.model)
            .bind(window.used_percent)
            .bind(window.remaining_percent)
            .bind(window.reset_at.map(|date| date.to_rfc3339()))
            .bind(window.derived_used_percent)
            .bind(window.derived_remaining_percent)
            .execute(&mut *tx)
            .await
            .map_err(|_| AppError::Database)?;
        }

        tx.commit().await.map_err(|_| AppError::Database)?;
        Ok(())
    }

    pub async fn latest_usage(
        &self,
        provider: ProviderId,
    ) -> Result<Option<ProviderUsage>, AppError> {
        let snapshot = sqlx::query(
            "SELECT * FROM usage_snapshots
             WHERE provider = ?
             ORDER BY captured_at DESC
             LIMIT 1",
        )
        .bind(serde_string(&provider)?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AppError::Database)?;

        let Some(snapshot) = snapshot else {
            return Ok(None);
        };

        self.usage_from_snapshot_row(snapshot).await.map(Some)
    }

    pub async fn save_settings(&self, settings: &AppSettings) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO app_settings (key, value_json, updated_at)
             VALUES ('app', ?, ?)
             ON CONFLICT(key) DO UPDATE SET value_json = excluded.value_json, updated_at = excluded.updated_at",
        )
        .bind(serde_json::to_string(settings).map_err(|_| AppError::Database)?)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|_| AppError::Database)?;
        Ok(())
    }

    pub async fn load_settings(&self) -> Result<AppSettings, AppError> {
        let row = sqlx::query("SELECT value_json FROM app_settings WHERE key = 'app'")
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| AppError::Database)?;

        let Some(row) = row else {
            return Ok(AppSettings::default());
        };

        serde_json::from_str(&row.get::<String, _>("value_json")).map_err(|_| AppError::Database)
    }

    pub async fn upsert_connection_from_usage(
        &self,
        usage: &ProviderUsage,
    ) -> Result<(), AppError> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO provider_connections (
                id, provider, connection_type, account_label, plan_name, status,
                reliability, last_success_at, last_attempt_at, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(provider, connection_type) DO UPDATE SET
                account_label = excluded.account_label,
                plan_name = excluded.plan_name,
                status = excluded.status,
                reliability = excluded.reliability,
                last_success_at = excluded.last_success_at,
                last_attempt_at = excluded.last_attempt_at,
                updated_at = excluded.updated_at",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(serde_string(&usage.provider)?)
        .bind(serde_string(&usage.connection_type)?)
        .bind(&usage.account_label)
        .bind(&usage.plan_name)
        .bind(serde_string(&ProviderStateKind::Connected)?)
        .bind(serde_string(&usage.reliability)?)
        .bind(usage.fetched_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|_| AppError::Database)?;
        Ok(())
    }

    pub async fn connection(
        &self,
        provider: ProviderId,
        connection_type: ConnectionType,
    ) -> Result<Option<ProviderConnection>, AppError> {
        let row = sqlx::query(
            "SELECT * FROM provider_connections WHERE provider = ? AND connection_type = ?",
        )
        .bind(serde_string(&provider)?)
        .bind(serde_string(&connection_type)?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AppError::Database)?;

        row.map(connection_from_row).transpose()
    }

    pub async fn mark_notification_sent(
        &self,
        provider: ProviderId,
        connection_type: ConnectionType,
        window_key: &str,
        threshold: u8,
        period_identifier: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            "INSERT OR IGNORE INTO notification_states (
                id, provider, connection_type, window_key, threshold, period_identifier, notified_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(serde_string(&provider)?)
        .bind(serde_string(&connection_type)?)
        .bind(window_key)
        .bind(threshold)
        .bind(period_identifier)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|_| AppError::Database)?;
        Ok(())
    }

    pub async fn notification_sent(
        &self,
        provider: ProviderId,
        connection_type: ConnectionType,
        window_key: &str,
        threshold: u8,
        period_identifier: &str,
    ) -> Result<bool, AppError> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM notification_states
             WHERE provider = ? AND connection_type = ? AND window_key = ?
             AND threshold = ? AND period_identifier = ?",
        )
        .bind(serde_string(&provider)?)
        .bind(serde_string(&connection_type)?)
        .bind(window_key)
        .bind(threshold)
        .bind(period_identifier)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::Database)?;

        Ok(row.0 > 0)
    }

    async fn usage_from_snapshot_row(
        &self,
        snapshot: sqlx::sqlite::SqliteRow,
    ) -> Result<ProviderUsage, AppError> {
        let snapshot_id: String = snapshot.get("id");
        let window_rows =
            sqlx::query("SELECT * FROM usage_windows WHERE snapshot_id = ? ORDER BY rowid")
                .bind(&snapshot_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|_| AppError::Database)?;
        let windows = window_rows
            .into_iter()
            .map(window_from_row)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ProviderUsage {
            id: snapshot_id,
            provider: serde_from_string(snapshot.get::<String, _>("provider"))?,
            connection_type: serde_from_string(snapshot.get::<String, _>("connection_type"))?,
            account_label: snapshot.get("account_label"),
            plan_name: snapshot.get("plan_name"),
            windows,
            token_usage: None,
            cost_usage: None,
            reliability: serde_from_string(snapshot.get::<String, _>("reliability"))?,
            fetched_at: parse_utc(snapshot.get::<String, _>("fetched_at"))?,
            stale: snapshot.get::<bool, _>("stale"),
            warnings: serde_json::from_str(&snapshot.get::<String, _>("warning_json"))
                .map_err(|_| AppError::Database)?,
        })
    }
}

fn window_from_row(row: sqlx::sqlite::SqliteRow) -> Result<UsageWindow, AppError> {
    Ok(UsageWindow {
        id: row.get("external_window_id"),
        label: row.get("label"),
        period: serde_from_string::<UsagePeriod>(row.get("period"))?,
        model: row.get("model"),
        used_percent: row.get("used_percent"),
        remaining_percent: row.get("remaining_percent"),
        reset_at: row
            .get::<Option<String>, _>("reset_at")
            .map(parse_utc)
            .transpose()?,
        derived_used_percent: row.get("derived_used_percent"),
        derived_remaining_percent: row.get("derived_remaining_percent"),
    })
}

fn connection_from_row(row: sqlx::sqlite::SqliteRow) -> Result<ProviderConnection, AppError> {
    Ok(ProviderConnection {
        id: row.get("id"),
        provider: serde_from_string(row.get("provider"))?,
        connection_type: serde_from_string(row.get("connection_type"))?,
        account_label: row.get("account_label"),
        plan_name: row.get("plan_name"),
        status: serde_from_string(row.get("status"))?,
        executable_path: row.get("executable_path"),
        cli_version: row.get("cli_version"),
        reliability: row
            .get::<Option<String>, _>("reliability")
            .map(serde_from_string::<Reliability>)
            .transpose()?,
        last_success_at: row
            .get::<Option<String>, _>("last_success_at")
            .map(parse_utc)
            .transpose()?,
        last_attempt_at: row
            .get::<Option<String>, _>("last_attempt_at")
            .map(parse_utc)
            .transpose()?,
        last_error: None,
        created_at: parse_utc(row.get("created_at"))?,
        updated_at: parse_utc(row.get("updated_at"))?,
    })
}

fn serde_string<T: serde::Serialize>(value: &T) -> Result<String, AppError> {
    match serde_json::to_value(value).map_err(|_| AppError::Database)? {
        serde_json::Value::String(value) => Ok(value),
        _ => Err(AppError::Database),
    }
}

fn serde_from_string<T: serde::de::DeserializeOwned>(value: String) -> Result<T, AppError> {
    serde_json::from_value(serde_json::Value::String(value)).map_err(|_| AppError::Database)
}

fn parse_utc(value: String) -> Result<DateTime<Utc>, AppError> {
    DateTime::parse_from_rfc3339(&value)
        .map(|date| date.with_timezone(&Utc))
        .map_err(|_| AppError::Database)
}

fn snapshots_match_meaningfully(left: &ProviderUsage, right: &ProviderUsage) -> bool {
    left.provider == right.provider
        && left.connection_type == right.connection_type
        && left.account_label == right.account_label
        && left.plan_name == right.plan_name
        && left.reliability == right.reliability
        && left.stale == right.stale
        && left.warnings == right.warnings
        && left.windows == right.windows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn creates_fresh_database() {
        let database = Database::in_memory().await.expect("database");
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM provider_connections")
            .fetch_one(database.pool())
            .await
            .expect("table count");

        assert_eq!(row.0, 0);
    }

    #[tokio::test]
    async fn migrations_can_run_more_than_once() {
        let database = Database::in_memory().await.expect("database");

        database.run_migrations().await.expect("rerun migrations");
    }

    #[tokio::test]
    async fn usage_window_percentage_constraint_rejects_invalid_values() {
        let database = Database::in_memory().await.expect("database");
        sqlx::query(
            "INSERT INTO usage_snapshots (id, provider, connection_type, reliability, fetched_at, captured_at, stale, warning_json)
             VALUES ('s1', 'codex', 'subscription-cli', 'official-cli-text', '2026-07-18T00:00:00Z', '2026-07-18T00:00:00Z', 0, '[]')",
        )
        .execute(database.pool())
        .await
        .expect("snapshot");

        let result = sqlx::query(
            "INSERT INTO usage_windows (id, snapshot_id, external_window_id, label, period, used_percent, derived_used_percent, derived_remaining_percent)
             VALUES ('w1', 's1', 'weekly', 'Weekly', 'weekly', 101, 0, 0)",
        )
        .execute(database.pool())
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn snapshot_repository_preserves_unknown_percentages() {
        let database = Database::in_memory().await.expect("database");
        let usage = sample_usage(None);

        database.save_usage_snapshot(&usage).await.expect("save");
        let latest = database
            .latest_usage(usage.provider)
            .await
            .expect("latest")
            .expect("usage");

        assert_eq!(latest.windows[0].used_percent, None);
    }

    #[tokio::test]
    async fn snapshot_repository_skips_duplicate_meaningful_snapshots() {
        let database = Database::in_memory().await.expect("database");
        let mut first = sample_usage(Some(28.0));
        let mut second = sample_usage(Some(28.0));
        second.id = "different-id".into();
        first.fetched_at = chrono::DateTime::parse_from_rfc3339("2026-07-18T00:00:00Z")
            .expect("date")
            .with_timezone(&chrono::Utc);
        second.fetched_at = chrono::DateTime::parse_from_rfc3339("2026-07-18T00:01:00Z")
            .expect("date")
            .with_timezone(&chrono::Utc);

        database.save_usage_snapshot(&first).await.expect("first");
        database.save_usage_snapshot(&second).await.expect("second");

        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM usage_snapshots")
            .fetch_one(database.pool())
            .await
            .expect("count");
        assert_eq!(row.0, 1);
    }

    #[tokio::test]
    async fn snapshot_repository_keeps_only_latest_per_provider() {
        let database = Database::in_memory().await.expect("database");
        let mut older = sample_usage(Some(10.0));
        older.fetched_at = chrono::DateTime::parse_from_rfc3339("2026-07-18T00:00:00Z")
            .expect("date")
            .with_timezone(&chrono::Utc);
        let mut newer = sample_usage(Some(20.0));
        newer.fetched_at = chrono::DateTime::parse_from_rfc3339("2026-07-18T01:00:00Z")
            .expect("date")
            .with_timezone(&chrono::Utc);

        database.save_usage_snapshot(&older).await.expect("older");
        database.save_usage_snapshot(&newer).await.expect("newer");

        let latest = database
            .latest_usage(ProviderId::Codex)
            .await
            .expect("latest")
            .expect("usage");
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM usage_snapshots")
            .fetch_one(database.pool())
            .await
            .expect("count");

        assert_eq!(latest.windows[0].used_percent, Some(20.0));
        assert_eq!(row.0, 1);
    }

    #[tokio::test]
    async fn settings_repository_updates_settings() {
        let database = Database::in_memory().await.expect("database");
        let mut settings = crate::domain::AppSettings::default();
        settings.refresh.interval_minutes = 10;

        database
            .save_settings(&settings)
            .await
            .expect("save settings");
        let loaded = database.load_settings().await.expect("load settings");

        assert_eq!(loaded.refresh.interval_minutes, 10);
    }

    #[tokio::test]
    async fn provider_metadata_can_be_updated() {
        let database = Database::in_memory().await.expect("database");
        let usage = sample_usage(Some(42.0));

        database
            .upsert_connection_from_usage(&usage)
            .await
            .expect("upsert");
        let connection = database
            .connection(usage.provider, usage.connection_type)
            .await
            .expect("connection")
            .expect("exists");

        assert_eq!(connection.provider, usage.provider);
        assert_eq!(connection.reliability, Some(usage.reliability));
    }

    #[tokio::test]
    async fn notification_state_repository_deduplicates_period_thresholds() {
        use crate::domain::{ConnectionType, ProviderId};

        let database = Database::in_memory().await.expect("database");

        database
            .mark_notification_sent(
                ProviderId::Codex,
                ConnectionType::SubscriptionCli,
                "weekly",
                80,
                "2026-W29",
            )
            .await
            .expect("mark");
        database
            .mark_notification_sent(
                ProviderId::Codex,
                ConnectionType::SubscriptionCli,
                "weekly",
                80,
                "2026-W29",
            )
            .await
            .expect("mark idempotent");

        assert!(database
            .notification_sent(
                ProviderId::Codex,
                ConnectionType::SubscriptionCli,
                "weekly",
                80,
                "2026-W29",
            )
            .await
            .expect("sent"));
    }

    fn sample_usage(used_percent: Option<f64>) -> crate::domain::ProviderUsage {
        use crate::domain::{
            ConnectionType, ProviderId, ProviderUsage, Reliability, UsagePeriod, UsageWindow,
        };

        let fetched_at = chrono::DateTime::parse_from_rfc3339("2026-07-18T00:00:00Z")
            .expect("date")
            .with_timezone(&chrono::Utc);
        let window = UsageWindow::new("weekly", "Weekly limit", UsagePeriod::Weekly)
            .and_then(|window| window.with_percentages(used_percent, None))
            .expect("window");

        ProviderUsage::new(
            ProviderId::Codex,
            ConnectionType::SubscriptionCli,
            Reliability::OfficialCliText,
            vec![window],
            fetched_at,
        )
    }
}
