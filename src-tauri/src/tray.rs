use crate::commands::ProviderState;
use crate::domain::{AppErrorCode, ProviderStateKind};
use tauri::Manager;

pub struct MainTrayIcon(pub tauri::tray::TrayIcon<tauri::Wry>);
pub struct CodexTrayMenuItem(pub tauri::menu::MenuItem<tauri::Wry>);

pub fn codex_loading_tray_label() -> String {
    "Codex Loading…".into()
}

fn remaining_percent(used_percent: f64) -> f64 {
    (100.0 - used_percent).clamp(0.0, 100.0)
}

pub fn codex_tray_label(state: &ProviderState) -> String {
    match state.status {
        ProviderStateKind::NotInstalled => "Codex CLI unavailable".into(),
        ProviderStateKind::Unsupported => "Codex CLI unsupported".into(),
        ProviderStateKind::AuthenticationRequired => "Codex Sign in required".into(),
        _ => match &state.usage {
            Some(usage) => usage
                .windows
                .first()
                .and_then(|window| window.used_percent)
                .map(|percent| format!("Codex — {}% remaining", remaining_percent(percent).round()))
                .unwrap_or_else(|| "Codex —".into()),
            None => match state.last_error.as_ref().map(|error| error.code) {
                Some(AppErrorCode::AuthenticationExpired) => "Codex Sign in required".into(),
                Some(AppErrorCode::ProviderUnavailable) => "Codex CLI unavailable".into(),
                Some(AppErrorCode::Unsupported) => "Codex CLI unsupported".into(),
                _ => "Codex —".into(),
            },
        },
    }
}

pub fn update_codex_menu_item(app: &tauri::AppHandle, state: &ProviderState) {
    if let Some(item) = app.try_state::<CodexTrayMenuItem>() {
        let _ = item.0.set_text(codex_tray_label(state));
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;
    use crate::commands::ProviderState;
    use crate::domain::{
        AppError, ConnectionType, ProviderId, ProviderUsage, Reliability, UsagePeriod, UsageWindow,
    };

    #[test]
    fn formats_initial_codex_loading_label() {
        assert_eq!(codex_loading_tray_label(), "Codex Loading…");
    }

    #[test]
    fn formats_known_unknown_and_auth_required_codex_tray_values() {
        let mut usage = ProviderUsage::new(
            ProviderId::Codex,
            ConnectionType::SubscriptionCli,
            Reliability::OfficialCliJson,
            vec![UsageWindow::new("primary", "Primary", UsagePeriod::Weekly)
                .and_then(|window| window.with_percentages(Some(28.0), None))
                .expect("window")],
            Utc::now(),
        );
        let mut state = ProviderState {
            provider: ProviderId::Codex,
            status: ProviderStateKind::Connected,
            usage: Some(usage.clone()),
            last_error: None,
            is_refreshing: false,
        };
        assert_eq!(codex_tray_label(&state), "Codex — 72% remaining");

        usage.windows[0].used_percent = None;
        state.usage = Some(usage);
        assert_eq!(codex_tray_label(&state), "Codex —");

        state.usage = None;
        state.last_error = Some(AppError::AuthenticationExpired.payload());
        assert_eq!(codex_tray_label(&state), "Codex Sign in required");
    }
}
