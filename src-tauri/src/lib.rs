pub mod app_state;
pub mod commands;
pub mod database;
pub mod domain;
pub mod events;
pub mod http;
pub mod process;
pub mod providers;
pub mod scheduler;
pub mod secrets;
pub mod tray;

use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

struct AutoHideGuard(AtomicBool);
pub(crate) struct ToggleMenuItem(tauri::menu::MenuItem<tauri::Wry>);

#[tauri::command]
fn set_auto_hide_guard(state: tauri::State<'_, AutoHideGuard>, value: bool) {
    state.0.store(value, Ordering::SeqCst);
}

#[tauri::command]
fn hide_window(window: tauri::WebviewWindow) {
    let _ = window.hide();
    if let Some(item) = window.app_handle().try_state::<ToggleMenuItem>() {
        let _ = item.0.set_text("Open AI Usage Dock");
    }
}

#[tauri::command]
fn set_tray_title(app: tauri::AppHandle, title: Option<String>) {
    if let Some(tray) = app.tray_by_id(tray::MAIN_TRAY_ID) {
        let _ = tray.set_title(title.as_deref());
    }
}

#[tauri::command]
fn get_autostart_state(app: tauri::AppHandle) -> Result<bool, String> {
    app.autolaunch().is_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_autostart_state(app: tauri::AppHandle, enable: bool) -> Result<(), String> {
    if enable {
        app.autolaunch().enable().map_err(|e| e.to_string())
    } else {
        app.autolaunch().disable().map_err(|e| e.to_string())
    }
}

pub(crate) fn shutdown_backend(app: &tauri::AppHandle) {
    if let Some(state) = app.try_state::<Arc<app_state::AppStateHandle>>() {
        tauri::async_runtime::block_on(async {
            if let Some(app_state) = state.state_if_ready() {
                app_state.scheduler.shutdown().await;
                app_state.providers.shutdown_all().await;
            }
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AutoHideGuard(AtomicBool::new(false)))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("usage-popup") {
                let _ = window.show();
                let _ = window.set_focus();
                if let Some(item) = app.try_state::<ToggleMenuItem>() {
                    let _ = item.0.set_text("Hide AI Usage Dock");
                }
            }
        }))
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_auto_hide_guard,
            hide_window,
            set_tray_title,
            get_autostart_state,
            set_autostart_state,
            commands::get_app_bootstrap,
            commands::list_providers,
            commands::get_provider_state,
            commands::refresh_provider,
            commands::refresh_all_providers,
            commands::start_provider_login,
            commands::get_settings,
            commands::update_settings,
            commands::save_provider_api_key,
            commands::delete_provider_api_key,
            commands::set_fake_provider_scenario
        ])
        .setup(|app| {
            eprintln!("[startup] Tauri setup started");
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            tray::create_main_tray(app)?;

            let state = Arc::new(app_state::AppStateHandle::new());
            app.manage(state.clone());

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                eprintln!("[startup] Starting background initialization");
                let database_path = match app_handle.path().app_data_dir() {
                    Ok(path) => path.join("ai-usage-dock.sqlite3"),
                    Err(_) => {
                        eprintln!("[startup] Database initialization failed: app data directory unavailable");
                        state.initialize_failed(domain::AppError::Database.payload());
                        return;
                    }
                };
                state
                    .initialize_production(database_path, app_handle.clone())
                    .await;
                match state.get().await {
                    Ok(app_state) => {
                        eprintln!("[startup] starting Codex provider refresh");
                        let result =
                            commands::refresh_provider_data(&app_state, domain::ProviderId::Codex)
                                .await;
                        if let Some(codex) = result.data {
                            tray::update_codex_menu_item(&app_handle, &codex);
                            eprintln!("[startup] Codex provider refresh completed");
                        } else if let Some(error) = result.error {
                            tray::update_codex_menu_item(
                                &app_handle,
                                &commands::ProviderState {
                                    provider: domain::ProviderId::Codex,
                                    status: domain::ProviderStateKind::Error,
                                    usage: None,
                                    last_error: Some(error),
                                    is_refreshing: false,
                                },
                            );
                            eprintln!("[startup] Codex provider refresh failed");
                        }
                    }
                    Err(error) => {
                        tray::update_codex_menu_item(
                            &app_handle,
                            &commands::ProviderState {
                                provider: domain::ProviderId::Codex,
                                status: domain::ProviderStateKind::Error,
                                usage: None,
                                last_error: Some(error),
                                is_refreshing: false,
                            },
                        );
                        eprintln!("[startup] backend initialization failed; Codex refresh skipped");
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                let _ = window.hide();
                api.prevent_close();
            }
            tauri::WindowEvent::Focused(focused) if !focused => {
                let state = window.state::<AutoHideGuard>();
                if !state.0.load(Ordering::SeqCst) {
                    let _ = window.hide();
                    if let Some(item) = window.app_handle().try_state::<ToggleMenuItem>() {
                        let _ = item.0.set_text("Open AI Usage Dock");
                    }
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
