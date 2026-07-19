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

use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_positioner::{Position, WindowExt};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

struct AutoHideGuard(AtomicBool);
struct ToggleMenuItem(tauri::menu::MenuItem<tauri::Wry>);

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
    if let Some(tray) = app.tray_by_id("main") {
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

fn shutdown_scheduler(app: &tauri::AppHandle) {
    if let Some(state) = app.try_state::<Arc<app_state::AppStateHandle>>() {
        if let Some(scheduler) = state.scheduler_if_ready() {
            tauri::async_runtime::block_on(scheduler.shutdown());
        }
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
            commands::get_usage_history,
            commands::save_provider_api_key,
            commands::delete_provider_api_key,
            commands::set_fake_provider_scenario
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let state = Arc::new(app_state::AppStateHandle::new());
            app.manage(state.clone());

            let toggle_i =
                MenuItem::with_id(app, "toggle", "Open AI Usage Dock", true, None::<&str>)?;
            app.manage(ToggleMenuItem(toggle_i.clone()));
            let refresh_i = MenuItem::with_id(app, "refresh", "Refresh All", true, None::<&str>)?;

            let codex_i = MenuItem::with_id(
                app,
                "codex",
                crate::tray::codex_loading_tray_label(),
                false,
                None::<&str>,
            )?;
            app.manage(crate::tray::CodexTrayMenuItem(codex_i.clone()));
            let claude_i =
                MenuItem::with_id(app, "claude", "Claude         --%", false, None::<&str>)?;
            let ag_i = MenuItem::with_id(
                app,
                "antigravity",
                "Antigravity    --%",
                false,
                None::<&str>,
            )?;

            let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let launch_i = CheckMenuItem::with_id(
                app,
                "launch_at_login",
                "Launch at Login",
                true,
                false,
                None::<&str>,
            )?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let separator = PredefinedMenuItem::separator(app)?;

            let menu = Menu::with_items(
                app,
                &[
                    &toggle_i,
                    &refresh_i,
                    &separator,
                    &codex_i,
                    &claude_i,
                    &ag_i,
                    &separator,
                    &settings_i,
                    &launch_i,
                    &quit_i,
                ],
            )?;

            let toggle_i_clone = toggle_i.clone();
            let toggle_i_tray = toggle_i.clone();

            let icon = app
                .default_window_icon()
                .cloned()
                .ok_or(tauri::Error::UnknownPath)?;

            let tray_icon = TrayIconBuilder::with_id("main")
                .tooltip("AI Usage Dock")
                .icon(icon)
                .icon_as_template(true)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| {
                    if event.id.as_ref() == "quit" {
                        shutdown_scheduler(app);
                        app.exit(0);
                    } else if event.id.as_ref() == "toggle" {
                        if let Some(window) = app.get_webview_window("usage-popup") {
                            let is_visible = window.is_visible().unwrap_or(false);
                            if is_visible {
                                let _ = window.hide();
                                let _ = toggle_i_clone.set_text("Open AI Usage Dock");
                            } else {
                                let _ = window.move_window(Position::TrayCenter);
                                let _ = window.show();
                                let _ = window.set_focus();
                                let _ = toggle_i_clone.set_text("Hide AI Usage Dock");
                            }
                        }
                    }
                })
                .on_tray_icon_event(move |tray, event| {
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("usage-popup") {
                            let is_visible = window.is_visible().unwrap_or(false);
                            if is_visible {
                                let _ = window.hide();
                                let _ = toggle_i_tray.set_text("Open AI Usage Dock");
                            } else {
                                let _ = window.move_window(Position::TrayCenter);
                                let _ = window.show();
                                let _ = window.set_focus();
                                let _ = toggle_i_tray.set_text("Hide AI Usage Dock");
                            }
                        }
                    }
                })
                .build(app)?;
            app.manage(crate::tray::MainTrayIcon(tray_icon));

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                eprintln!("[startup] desktop shell ready; starting backend initialization");
                let database_path = match app_handle.path().app_data_dir() {
                    Ok(path) => path.join("ai-usage-dock.sqlite3"),
                    Err(_) => {
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
