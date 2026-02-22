use crate::models::growth::LevelInfo;

#[tauri::command]
pub async fn get_level_info() -> Result<LevelInfo, String> {
    // TODO: storage에서 레벨 데이터 로드
    Ok(LevelInfo {
        level: 1,
        current_exp: 0,
        exp_to_next: 60,
        total_exp: 0,
    })
}

#[tauri::command]
pub async fn get_exp_breakdown() -> Result<serde_json::Value, String> {
    // TODO: 오늘 EXP 소스별 내역
    Ok(serde_json::json!({
        "coding": 0,
        "commits": 0,
        "pomodoro": 0,
        "streak": 0,
        "total": 0
    }))
}
