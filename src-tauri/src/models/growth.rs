use serde::{Deserialize, Serialize};

/// 레벨 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelInfo {
    pub level: u32,
    pub current_exp: u32,
    pub exp_to_next: u32,
    pub total_exp: u32,
}

/// EXP 배율 상수
pub const EXP_PER_CODING_MINUTE: u32 = 1;
pub const EXP_PER_COMMIT: u32 = 20;
pub const EXP_PER_POMODORO: u32 = 30;

/// 하루 최대 보너스 커밋 수 (이후 20%로 감쇠)
pub const DAILY_COMMIT_CAP: u32 = 20;
pub const COMMIT_DECAY_RATE: f32 = 0.2;

/// 레벨별 필요 EXP 계산
/// Level 1→2: 60, Level 2→3: 150, ...
/// 공식: base * level^1.5 (초반 빠른 레벨업)
pub fn exp_for_level(level: u32) -> u32 {
    let base = 60.0_f64;
    (base * (level as f64).powf(1.3)).round() as u32
}

/// 커밋 EXP 계산 (악용 방지)
pub fn commit_exp(commit_count: u32) -> u32 {
    if commit_count <= DAILY_COMMIT_CAP {
        commit_count * EXP_PER_COMMIT
    } else {
        let base = DAILY_COMMIT_CAP * EXP_PER_COMMIT;
        let extra = ((commit_count - DAILY_COMMIT_CAP) as f32 * EXP_PER_COMMIT as f32 * COMMIT_DECAY_RATE) as u32;
        base + extra
    }
}

/// Streak 보너스 계산
pub fn streak_bonus(streak_days: u32) -> u32 {
    match streak_days {
        0..=1 => 0,
        2..=6 => 5,
        7..=13 => 15,
        14..=29 => 30,
        _ => 50,
    }
}
