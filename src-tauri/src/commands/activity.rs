use crate::models::activity::{CodingStatus, DailySummary};

/// 오늘 활동 요약
#[tauri::command]
pub async fn get_today_summary() -> Result<DailySummary, String> {
    // TODO: storage에서 오늘 데이터 로드
    Ok(DailySummary::default())
}

/// 현재 코딩 상태
#[tauri::command]
pub async fn get_coding_status() -> Result<CodingStatus, String> {
    // TODO: activity monitor에서 현재 상태
    Ok(CodingStatus {
        is_coding: false,
        active_ide: None,
        idle_seconds: 0,
        session_minutes: 0,
    })
}
