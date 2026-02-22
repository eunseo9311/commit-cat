use tauri::{AppHandle, Emitter};
use std::time::Duration;

/// IDE 프로세스 이름 목록
const IDE_PROCESSES: &[(&str, &str)] = &[
    ("code", "VS Code"),
    ("Code", "VS Code"),
    ("Code - Insiders", "VS Code Insiders"),
    ("idea", "IntelliJ IDEA"),
    ("idea64", "IntelliJ IDEA"),
    ("webstorm", "WebStorm"),
    ("webstorm64", "WebStorm"),
    ("pycharm", "PyCharm"),
    ("pycharm64", "PyCharm"),
    ("goland", "GoLand"),
    ("clion", "CLion"),
    ("rustrover", "RustRover"),
];

/// 백그라운드 활동 모니터 (이벤트 루프)
pub async fn start_monitor(app: AppHandle) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));

    loop {
        interval.tick().await;

        // 1. IDE 프로세스 감지
        if let Some(ide) = detect_running_ide() {
            let _ = app.emit("activity:ide-detected", &ide);
        }

        // 2. 풀스크린 감지
        if is_fullscreen() {
            let _ = app.emit("activity:fullscreen", true);
        }
    }
}

/// 실행 중인 IDE 감지
fn detect_running_ide() -> Option<String> {
    // TODO: OS별 프로세스 목록 조회
    None
}

/// 풀스크린 여부 감지
fn is_fullscreen() -> bool {
    // TODO: OS별 구현
    false
}
