use serde::{Deserialize, Serialize};

/// 앱 설정 (권한 토글 포함)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    /// 풀스크린 자동 숨김
    pub auto_hide_fullscreen: bool,
    /// 키보드/마우스 활동 감지 허용
    pub activity_tracking: bool,
    /// IDE 감지 허용
    pub ide_detection: bool,
    /// Git 연동 허용
    pub git_integration: bool,
    /// Docker 감지 허용 (v2)
    pub docker_integration: bool,
    /// AI 기능 (v3)
    pub ai_enabled: bool,
    /// 뽀모도로 기본 시간 (분)
    pub pomodoro_minutes: u32,
    /// 유휴 판정 시간 (초)
    pub idle_threshold_seconds: u64,
    /// 밤 시간 시작 (시, 24h)
    pub night_hour_start: u32,
    /// 밤 시간 끝 (시, 24h)
    pub night_hour_end: u32,
    /// 등록된 Git 저장소 경로들
    pub git_repos: Vec<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_hide_fullscreen: true,
            activity_tracking: true,  // MVP에서는 기본 ON
            ide_detection: true,
            git_integration: true,
            docker_integration: false, // v2
            ai_enabled: false,         // v3
            pomodoro_minutes: 25,
            idle_threshold_seconds: 300, // 5분
            night_hour_start: 23,
            night_hour_end: 6,
            git_repos: vec![],
        }
    }
}

/// 로컬 저장 전체 데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppData {
    pub version: u32,
    pub settings: AppSettings,
    pub cat: CatPersistence,
    pub today: super::activity::DailySummary,
    pub history: Vec<super::activity::DailySummary>,
}

/// 고양이 영구 데이터 (레벨/경험치)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatPersistence {
    pub level: u32,
    pub exp: u32,
    pub total_coding_minutes: u32,
    pub total_commits: u32,
    pub streak_days: u32,
    pub last_active_date: Option<String>,
}

impl Default for CatPersistence {
    fn default() -> Self {
        Self {
            level: 1,
            exp: 0,
            total_coding_minutes: 0,
            total_commits: 0,
            streak_days: 0,
            last_active_date: None,
        }
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            version: 1,
            settings: AppSettings::default(),
            cat: CatPersistence::default(),
            today: super::activity::DailySummary::default(),
            history: vec![],
        }
    }
}
