mod commands;
mod events;
mod models;
mod services;
mod utils;

use tauri::Manager;

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSColor, NSWindow};
#[cfg(target_os = "macos")]
use cocoa::base::{id, nil, NO};

#[cfg(target_os = "macos")]
fn setup_macos_window(window: &tauri::WebviewWindow) {
    use tauri::Emitter;

    if let Ok(ns_window) = window.ns_window() {
        unsafe {
            let ns_win = ns_window as id;
            // 윈도우를 불투명하지 않게 설정
            ns_win.setOpaque_(NO);
            // 배경색을 완전 투명으로 설정
            ns_win.setBackgroundColor_(NSColor::clearColor(nil));
            // 그림자 비활성화 (잔상 방지)
            ns_win.setHasShadow_(NO);
        }
    }
}

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

            // macOS 투명 윈도우 설정
            #[cfg(target_os = "macos")]
            if let Some(window) = app.get_webview_window("cat-overlay") {
                setup_macos_window(&window);
            }

            // 활동 모니터 시작
            let monitor_handle = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                services::activity::start_monitor(monitor_handle).await;
            });

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
