use serde::Serialize;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

/// 프론트엔드로 보내는 활동 상태
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityStatus {
    pub is_ide_running: bool,
    pub active_ide: Option<String>,
    pub idle_seconds: u64,
}

/// 백그라운드 활동 모니터
pub async fn start_monitor(app: AppHandle) {
    let mut interval = tokio::time::interval(Duration::from_secs(10));
    let mut last_ide_seen = Instant::now();
    let mut was_ide_running = false;
    let mut sleep_emitted = false;

    loop {
        interval.tick().await;

        // 1. IDE 프로세스 감지
        let detected_ide = detect_running_ide();
        let is_ide_running = detected_ide.is_some();

        if is_ide_running {
            last_ide_seen = Instant::now();
            sleep_emitted = false;
        }

        let idle_seconds = last_ide_seen.elapsed().as_secs();

        // 2. 상태 변화 시에만 이벤트 발생
        if is_ide_running && !was_ide_running {
            let ide_name = detected_ide.clone().unwrap_or("IDE".to_string());
            let _ = app.emit("activity:ide-detected", &ide_name);
        } else if !is_ide_running && was_ide_running {
            let _ = app.emit("activity:ide-closed", "");
        }

        // 3. 유휴 시간 체크
        if idle_seconds >= 600 && !sleep_emitted {
            let _ = app.emit("activity:sleeping", idle_seconds);
            sleep_emitted = true;
        } else if idle_seconds >= 180 && idle_seconds < 600 && was_ide_running && !is_ide_running {
            let _ = app.emit("activity:idle", idle_seconds);
        }

        // 4. 밤 시간 체크
        let hour = chrono::Local::now().hour();
        if is_ide_running && (hour >= 23 || hour < 6) {
            let _ = app.emit("activity:late-night-coding", hour);
        }

        was_ide_running = is_ide_running;

        // 5. 주기적 상태 보고
        let status = ActivityStatus {
            is_ide_running,
            active_ide: detected_ide,
            idle_seconds,
        };
        let _ = app.emit("activity:status", &status);
    }
}

use chrono::Timelike;

/// IDE 감지 (OS별 분기)
fn detect_running_ide() -> Option<String> {
    #[cfg(target_os = "macos")]
    { detect_running_ide_macos() }

    #[cfg(target_os = "windows")]
    { detect_running_ide_windows() }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    { detect_running_ide_unix() }
}

// ═══════════════════════════════════════
// macOS: 프로세스 전체 경로로 감지
// ═══════════════════════════════════════
#[cfg(target_os = "macos")]
fn detect_running_ide_macos() -> Option<String> {
    // ps -A -o args= 로 전체 커맨드라인을 가져옴
    let output = std::process::Command::new("ps")
        .args(["-A", "-o", "args="])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // 앱 경로 패턴 → IDE 이름
    let app_patterns: &[(&str, &str)] = &[
        // VS Code 계열
        ("Visual Studio Code.app", "VS Code"),
        ("Visual Studio Code - Insiders.app", "VS Code Insiders"),
        ("Cursor.app", "Cursor"),
        ("Windsurf.app", "Windsurf"),
        // JetBrains 계열
        ("IntelliJ IDEA.app", "IntelliJ IDEA"),
        ("IntelliJ IDEA CE.app", "IntelliJ IDEA"),
        ("WebStorm.app", "WebStorm"),
        ("PyCharm.app", "PyCharm"),
        ("PyCharm CE.app", "PyCharm"),
        ("GoLand.app", "GoLand"),
        ("CLion.app", "CLion"),
        ("RustRover.app", "RustRover"),
        ("DataGrip.app", "DataGrip"),
        ("Rider.app", "Rider"),
        // 기타
        ("Xcode.app", "Xcode"),
        ("Zed.app", "Zed"),
        ("Sublime Text.app", "Sublime Text"),
        ("Android Studio.app", "Android Studio"),
    ];

    for (pattern, ide_name) in app_patterns {
        if stdout.contains(pattern) {
            return Some(ide_name.to_string());
        }
    }

    None
}

// ═══════════════════════════════════════
// Windows: tasklist로 감지
// ═══════════════════════════════════════
#[cfg(target_os = "windows")]
fn detect_running_ide_windows() -> Option<String> {
    let output = std::process::Command::new("tasklist")
        .args(["/FO", "CSV", "/NH"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let win_processes: &[(&str, &str)] = &[
        ("Code.exe", "VS Code"),
        ("Cursor.exe", "Cursor"),
        ("devenv.exe", "Visual Studio"),
        ("idea64.exe", "IntelliJ IDEA"),
        ("webstorm64.exe", "WebStorm"),
        ("pycharm64.exe", "PyCharm"),
        ("goland64.exe", "GoLand"),
        ("clion64.exe", "CLion"),
        ("rustrover64.exe", "RustRover"),
        ("datagrip64.exe", "DataGrip"),
        ("rider64.exe", "Rider"),
    ];

    for (proc_name, ide_name) in win_processes {
        if stdout.contains(proc_name) {
            return Some(ide_name.to_string());
        }
    }

    None
}

// ═══════════════════════════════════════
// Linux: ps 기반
// ═══════════════════════════════════════
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn detect_running_ide_unix() -> Option<String> {
    let output = std::process::Command::new("ps")
        .args(["-A", "-o", "args="])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let patterns: &[(&str, &str)] = &[
        ("/code", "VS Code"),
        ("/cursor", "Cursor"),
        ("idea", "IntelliJ IDEA"),
        ("webstorm", "WebStorm"),
        ("pycharm", "PyCharm"),
        ("goland", "GoLand"),
        ("clion", "CLion"),
        ("rustrover", "RustRover"),
        ("sublime_text", "Sublime Text"),
    ];

    for (pattern, ide_name) in patterns {
        if stdout.lines().any(|l| l.contains(pattern)) {
            return Some(ide_name.to_string());
        }
    }

    None
}
