use serde::Serialize;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

/// IDE 프로세스 이름 → 표시 이름
const IDE_PROCESSES: &[(&str, &str)] = &[
    // VS Code
    ("Electron", "VS Code"),  // macOS에서 VS Code 프로세스명
    ("Code Helper", "VS Code"),
    ("code", "VS Code"),
    ("Code", "VS Code"),
    ("Code - Insiders", "VS Code Insiders"),
    // JetBrains
    ("idea", "IntelliJ IDEA"),
    ("IntelliJ IDEA", "IntelliJ IDEA"),
    ("webstorm", "WebStorm"),
    ("WebStorm", "WebStorm"),
    ("pycharm", "PyCharm"),
    ("PyCharm", "PyCharm"),
    ("goland", "GoLand"),
    ("GoLand", "GoLand"),
    ("clion", "CLion"),
    ("CLion", "CLion"),
    ("RustRover", "RustRover"),
    ("rustrover", "RustRover"),
    // 기타
    ("Xcode", "Xcode"),
    ("Cursor", "Cursor"),
    ("cursor", "Cursor"),
    ("Windsurf", "Windsurf"),
    ("zed", "Zed"),
    ("Zed", "Zed"),
];

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
            // IDE 켜짐 → coding
            let ide_name = detected_ide.clone().unwrap_or("IDE".to_string());
            let _ = app.emit("activity:ide-detected", &ide_name);
        } else if !is_ide_running && was_ide_running {
            // IDE 꺼짐 → idle
            let _ = app.emit("activity:ide-closed", "");
        }

        // 3. 유휴 시간 체크
        if idle_seconds >= 600 && !sleep_emitted {
            // 10분 이상 무활동 → sleeping
            let _ = app.emit("activity:sleeping", idle_seconds);
            sleep_emitted = true;
        } else if idle_seconds >= 180 && idle_seconds < 600 && was_ide_running && !is_ide_running {
            // 3분 이상 IDE 없음 → idle
            let _ = app.emit("activity:idle", idle_seconds);
        }

        // 4. 밤 시간 체크
        let hour = chrono::Local::now().hour();
        if is_ide_running && (hour >= 23 || hour < 6) {
            let _ = app.emit("activity:late-night-coding", hour);
        }

        was_ide_running = is_ide_running;

        // 5. 주기적 상태 보고 (프론트엔드 동기화용)
        let status = ActivityStatus {
            is_ide_running,
            active_ide: detected_ide,
            idle_seconds,
        };
        let _ = app.emit("activity:status", &status);
    }
}

use chrono::Timelike;

/// macOS/Linux: ps 기반 IDE 프로세스 감지
fn detect_running_ide() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        detect_running_ide_macos()
    }

    #[cfg(target_os = "windows")]
    {
        detect_running_ide_windows()
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        detect_running_ide_unix()
    }
}

#[cfg(target_os = "macos")]
fn detect_running_ide_macos() -> Option<String> {
    // macOS: osascript로 활성 앱 이름 확인 (가장 정확)
    let active_app = std::process::Command::new("osascript")
        .args(["-e", r#"tell application "System Events" to get name of first application process whose frontmost is true"#])
        .output()
        .ok()
        .and_then(|out| {
            if out.status.success() {
                Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
            } else {
                None
            }
        });

    if let Some(ref app_name) = active_app {
        // 활성 앱이 IDE인지 확인
        for (proc_name, ide_name) in IDE_PROCESSES {
            if app_name.contains(proc_name) {
                return Some(ide_name.to_string());
            }
        }
    }

    // fallback: ps로 IDE 프로세스가 실행 중인지 확인
    let output = std::process::Command::new("ps")
        .args(["-A", "-o", "comm="])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for (proc_name, ide_name) in IDE_PROCESSES {
        for line in stdout.lines() {
            let binary = line.rsplit('/').next().unwrap_or(line).trim();
            if binary == *proc_name {
                return Some(ide_name.to_string());
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn detect_running_ide_windows() -> Option<String> {
    let output = std::process::Command::new("tasklist")
        .args(["/FO", "CSV", "/NH"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let win_processes: &[(&str, &str)] = &[
        ("Code.exe", "VS Code"),
        ("devenv.exe", "Visual Studio"),
        ("idea64.exe", "IntelliJ IDEA"),
        ("webstorm64.exe", "WebStorm"),
        ("pycharm64.exe", "PyCharm"),
        ("goland64.exe", "GoLand"),
        ("clion64.exe", "CLion"),
        ("rustrover64.exe", "RustRover"),
        ("Cursor.exe", "Cursor"),
    ];

    for (proc_name, ide_name) in win_processes {
        if stdout.contains(proc_name) {
            return Some(ide_name.to_string());
        }
    }

    None
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn detect_running_ide_unix() -> Option<String> {
    let output = std::process::Command::new("ps")
        .args(["-A", "-o", "comm="])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for (proc_name, ide_name) in IDE_PROCESSES {
        if stdout.lines().any(|l| l.trim().ends_with(proc_name)) {
            return Some(ide_name.to_string());
        }
    }

    None
}
