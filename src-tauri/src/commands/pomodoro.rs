use crate::models::activity::PomodoroStatus;

#[tauri::command]
pub async fn start_pomodoro() -> Result<PomodoroStatus, String> {
    // TODO: 타이머 시작, 상태 업데이트
    Ok(PomodoroStatus {
        is_active: true,
        remaining_seconds: 25 * 60,
        total_seconds: 25 * 60,
        sessions_today: 0,
    })
}

#[tauri::command]
pub async fn stop_pomodoro() -> Result<PomodoroStatus, String> {
    // TODO: 타이머 중지
    Ok(PomodoroStatus {
        is_active: false,
        remaining_seconds: 0,
        total_seconds: 25 * 60,
        sessions_today: 0,
    })
}

#[tauri::command]
pub async fn get_pomodoro_status() -> Result<PomodoroStatus, String> {
    // TODO: 현재 뽀모도로 상태
    Ok(PomodoroStatus {
        is_active: false,
        remaining_seconds: 0,
        total_seconds: 25 * 60,
        sessions_today: 0,
    })
}
