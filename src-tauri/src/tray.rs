use crate::commands::ProviderState;
use crate::domain::{AppErrorCode, ProviderStateKind};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_positioner::{Position, WindowExt};

pub const MAIN_TRAY_ID: &str = "main-tray";

pub struct MainTrayIcon(pub tauri::tray::TrayIcon<tauri::Wry>);
pub struct CodexTrayMenuItem(pub tauri::menu::MenuItem<tauri::Wry>);

pub fn main_tray_icon() -> tauri::Result<tauri::image::Image<'static>> {
    let mut rgba = vec![0_u8; 32 * 32 * 4];

    for y in 0_i32..32 {
        for x in 0_i32..32 {
            let dx = x - 15;
            let dy = y - 15;
            let distance_squared = dx * dx + dy * dy;
            let in_outer_disc = distance_squared <= 13 * 13;
            let in_center_cutout = distance_squared <= 3 * 3;
            let in_status_cutout = (20..=23).contains(&x) && (8..=11).contains(&y);

            if in_outer_disc && !in_center_cutout && !in_status_cutout {
                let index = ((y * 32 + x) * 4) as usize;
                rgba[index..index + 4].copy_from_slice(&[255, 255, 255, 255]);
            }
        }
    }

    Ok(tauri::image::Image::new_owned(rgba, 32, 32))
}

pub fn create_main_tray(app: &mut tauri::App) -> tauri::Result<()> {
    if app.tray_by_id(MAIN_TRAY_ID).is_some() {
        eprintln!("[startup] Main tray already exists; skipping duplicate creation");
        return Ok(());
    }

    eprintln!("[startup] Loading tray icon: built-in 32x32 macOS template mask");
    let tray_icon = main_tray_icon().map_err(|error| {
        eprintln!(
            "[startup] Failed to load tray icon from built-in 32x32 macOS template mask: {error}"
        );
        error
    })?;

    let toggle_i = MenuItem::with_id(app, "toggle", "Open AI Usage Dock", true, None::<&str>)?;
    app.manage(crate::ToggleMenuItem(toggle_i.clone()));
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&toggle_i, &quit_i])?;

    let toggle_i_menu = toggle_i.clone();
    let toggle_i_tray = toggle_i.clone();

    eprintln!("[startup] Creating main tray");
    let tray_icon = TrayIconBuilder::with_id(MAIN_TRAY_ID)
        .icon(tray_icon)
        .icon_as_template(true)
        .tooltip("AI Usage Dock")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                crate::shutdown_backend(app);
                app.exit(0);
            }
            "toggle" => toggle_popup(app, &toggle_i_menu),
            _ => {}
        })
        .on_tray_icon_event(move |tray, event| {
            tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
            if let tauri::tray::TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                button_state: tauri::tray::MouseButtonState::Up,
                ..
            } = event
            {
                toggle_popup(tray.app_handle(), &toggle_i_tray);
            }
        })
        .build(app)
        .map_err(|error| {
            eprintln!("[startup] Failed to create main tray with id {MAIN_TRAY_ID}: {error}");
            error
        })?;

    app.manage(MainTrayIcon(tray_icon));
    eprintln!("[startup] Main tray created successfully");
    Ok(())
}

fn toggle_popup(app: &tauri::AppHandle, toggle_item: &MenuItem<tauri::Wry>) {
    if let Some(window) = app.get_webview_window("usage-popup") {
        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            let _ = window.hide();
            let _ = toggle_item.set_text("Open AI Usage Dock");
        } else {
            let _ = window.move_window(Position::TrayCenter);
            let _ = window.show();
            let _ = window.set_focus();
            let _ = toggle_item.set_text("Hide AI Usage Dock");
        }
    }
}

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
    fn bundled_tray_icon_loads() {
        let icon = main_tray_icon().expect("tray icon");

        assert_eq!(icon.width(), 32);
        assert_eq!(icon.height(), 32);
    }

    #[test]
    fn bundled_tray_icon_is_template_safe() {
        let icon = main_tray_icon().expect("tray icon");
        let visible_pixels = icon
            .rgba()
            .chunks_exact(4)
            .filter(|pixel| pixel[3] > 0)
            .collect::<Vec<_>>();

        assert!(
            visible_pixels.len() >= 400,
            "tray icon must have enough visible pixels for the macOS menu bar"
        );
        assert!(
            visible_pixels
                .iter()
                .all(|pixel| pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255),
            "macOS template tray icon should be a white alpha mask"
        );
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
