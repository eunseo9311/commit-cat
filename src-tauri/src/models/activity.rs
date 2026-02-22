use serde::{Deserialize, Serialize};

/// 오늘 하루 요약
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DailySummary {
    pub date: String,                // "2026-02-22"
    pub coding_minutes: u32,
    pub commits: u32,
    pub pomodoro_sessions: u32,
    pub exp_gained: u32,
}

/// 실시간 코딩 상태
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodingStatus {
    pub is_coding: bool,
    pub active_ide: Option<String>,  // "VS Code" | "IntelliJ" | null
    pub idle_seconds: u64,
    pub session_minutes: u32,
}

/// 뽀모도로 상태
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PomodoroStatus {
    pub is_active: bool,
    pub remaining_seconds: u32,
    pub total_seconds: u32,
    pub sessions_today: u32,
}
