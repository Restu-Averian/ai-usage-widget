use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ConnectionType, DomainError, ProviderId, Reliability};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UsagePeriod {
    Session,
    FiveHour,
    Daily,
    Weekly,
    Monthly,
    ModelSpecific,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageWindow {
    pub id: String,
    pub label: String,
    pub period: UsagePeriod,
    pub model: Option<String>,
    pub used_percent: Option<f64>,
    pub remaining_percent: Option<f64>,
    pub reset_at: Option<DateTime<Utc>>,
    pub derived_used_percent: bool,
    pub derived_remaining_percent: bool,
}

impl UsageWindow {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        period: UsagePeriod,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        let label = label.into();
        if id.trim().is_empty() {
            return Err(DomainError::EmptyField("id"));
        }
        if label.trim().is_empty() {
            return Err(DomainError::EmptyField("label"));
        }

        Ok(Self {
            id,
            label,
            period,
            model: None,
            used_percent: None,
            remaining_percent: None,
            reset_at: None,
            derived_used_percent: false,
            derived_remaining_percent: false,
        })
    }

    pub fn with_percentages(
        mut self,
        used_percent: Option<f64>,
        remaining_percent: Option<f64>,
    ) -> Result<Self, DomainError> {
        validate_percentage("used_percent", used_percent)?;
        validate_percentage("remaining_percent", remaining_percent)?;

        self.used_percent = used_percent;
        self.remaining_percent = remaining_percent;

        if self.used_percent.is_none() {
            if let Some(remaining) = self.remaining_percent {
                self.used_percent = Some(100.0 - remaining);
                self.derived_used_percent = true;
            }
        }

        if self.remaining_percent.is_none() {
            if let Some(used) = self.used_percent {
                self.remaining_percent = Some(100.0 - used);
                self.derived_remaining_percent = true;
            }
        }

        Ok(self)
    }

    pub fn with_reset_at(mut self, reset_at: DateTime<Utc>) -> Self {
        self.reset_at = Some(reset_at);
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
}

fn validate_percentage(field: &'static str, value: Option<f64>) -> Result<(), DomainError> {
    if let Some(value) = value {
        if !(0.0..=100.0).contains(&value) || !value.is_finite() {
            return Err(DomainError::InvalidPercentage { field, value });
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UsageWarning {
    Offline,
    Stale,
    PercentageUnavailable,
    ExperimentalParser,
    ResetTimePassed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenUsage {
    pub input: u64,
    pub output: u64,
    pub cached: Option<u64>,
    pub requests: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostUsage {
    pub used: f64,
    pub budget: Option<f64>,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUsage {
    pub id: String,
    pub provider: ProviderId,
    pub connection_type: ConnectionType,
    pub account_label: Option<String>,
    pub plan_name: Option<String>,
    pub windows: Vec<UsageWindow>,
    pub token_usage: Option<TokenUsage>,
    pub cost_usage: Option<CostUsage>,
    pub reliability: Reliability,
    pub fetched_at: DateTime<Utc>,
    pub stale: bool,
    pub warnings: Vec<UsageWarning>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryPoint {
    pub timestamp: DateTime<Utc>,
    pub value: Option<f64>,
}

impl ProviderUsage {
    pub fn new(
        provider: ProviderId,
        connection_type: ConnectionType,
        reliability: Reliability,
        windows: Vec<UsageWindow>,
        fetched_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            provider,
            connection_type,
            account_label: None,
            plan_name: None,
            windows,
            token_usage: None,
            cost_usage: None,
            reliability,
            fetched_at,
            stale: false,
            warnings: Vec::new(),
        }
    }

    pub fn with_account(
        mut self,
        account_label: impl Into<String>,
        plan_name: impl Into<String>,
    ) -> Self {
        self.account_label = Some(account_label.into());
        self.plan_name = Some(plan_name.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_window_preserves_unknown_percentages() {
        let window =
            UsageWindow::new("weekly", "Weekly limit", UsagePeriod::Weekly).expect("valid window");

        assert_eq!(window.used_percent, None);
        assert_eq!(window.remaining_percent, None);
    }

    #[test]
    fn usage_window_rejects_invalid_percentages() {
        let result = UsageWindow::new("weekly", "Weekly limit", UsagePeriod::Weekly)
            .and_then(|window| window.with_percentages(Some(101.0), None));

        assert!(matches!(
            result,
            Err(DomainError::InvalidPercentage {
                field: "used_percent",
                value: 101.0
            })
        ));
    }
}
