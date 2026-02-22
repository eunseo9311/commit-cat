mod commands;
mod events;
mod models;
mod services;
mod utils;

use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // ── Plugins ──
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        // ── State ──
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Initialize local data storage
            services::storage::init(&app_handle)?;

            // Start background services (MVP: disabled for now)
            // TODO: Enable background services with proper async runtime
            // let handle = app_handle.clone();
            // tauri::async_runtime::spawn(async move {
            //     services::activity::start_monitor(handle).await;
            // });
            // let handle = app_handle.clone();
            // tauri::async_runtime::spawn(async move {
            //     services::git::start_watcher(handle).await;
            // });
            let _ = app_handle; // suppress unused warning

            Ok(())
        })
        // ── Commands (frontend ↔ backend) ──
        .invoke_handler(tauri::generate_handler![
            // Cat state
            commands::cat::get_cat_state,
            commands::cat::click_cat,
            // Activity
            commands::activity::get_today_summary,
            commands::activity::get_coding_status,
            // Growth
            commands::growth::get_level_info,
            commands::growth::get_exp_breakdown,
            // Pomodoro
            commands::pomodoro::start_pomodoro,
            commands::pomodoro::stop_pomodoro,
            commands::pomodoro::get_pomodoro_status,
            // Git
            commands::git::get_today_commits,
            commands::git::register_repo,
            // Settings
            commands::settings::get_settings,
            commands::settings::update_settings,
            // Fullscreen
            commands::fullscreen::check_fullscreen,
        ])
        // ── Tray icon click handler ──
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click { .. } = event {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("tray-panel") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Commit Cat");
}
