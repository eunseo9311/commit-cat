use crate::models::cat::CatState;
use chrono::Local;

/// 상태 전환 조건 (초 단위 기준값)
pub const IDLE_TO_SLEEPING_SECS: u64 = 600;    // 10분 무활동 → 수면
pub const CODING_TO_IDLE_SECS: u64 = 180;      // 3분 무활동 → idle
pub const CELEBRATING_DURATION_SECS: u64 = 5;  // 5초 후 idle 복귀
pub const FRUSTRATED_DURATION_SECS: u64 = 5;   // 5초 후 idle 복귀

/// 상태 전환 입력 이벤트
#[derive(Debug)]
pub enum StateEvent {
    ActivityDetected,
    IdleTimeout(u64),    // 유휴 시간 (초)
    CommitDetected,
    ErrorDetected,
    UserClicked,
    TimerExpired,        // Celebrating/Frustrated 자동 복귀
}

/// 상태 전환 로직
pub fn transition(current: &CatState, event: &StateEvent) -> Option<CatState> {
    match (current, event) {
        // ── Idle ──
        (CatState::Idle, StateEvent::ActivityDetected) => Some(CatState::Coding),
        (CatState::Idle, StateEvent::IdleTimeout(secs)) if *secs >= IDLE_TO_SLEEPING_SECS => {
            if is_night_time() {
                Some(CatState::Sleeping)
            } else {
                Some(CatState::Sleeping)
            }
        }
        (CatState::Idle, StateEvent::UserClicked) => Some(CatState::Interaction),

        // ── Coding ──
        (CatState::Coding, StateEvent::CommitDetected) => Some(CatState::Celebrating),
        (CatState::Coding, StateEvent::ErrorDetected) => Some(CatState::Frustrated),
        (CatState::Coding, StateEvent::IdleTimeout(secs)) if *secs >= CODING_TO_IDLE_SECS => {
            Some(CatState::Idle)
        }
        (CatState::Coding, StateEvent::UserClicked) => Some(CatState::Interaction),

        // ── Celebrating → Idle ──
        (CatState::Celebrating, StateEvent::TimerExpired) => Some(CatState::Idle),

        // ── Frustrated → Idle ──
        (CatState::Frustrated, StateEvent::TimerExpired) => Some(CatState::Idle),

        // ── Sleeping → Idle ──
        (CatState::Sleeping, StateEvent::ActivityDetected) => Some(CatState::Idle),
        (CatState::Sleeping, StateEvent::UserClicked) => Some(CatState::Idle),

        // ── Interaction → Idle ──
        (CatState::Interaction, StateEvent::TimerExpired) => Some(CatState::Idle),

        // ── Tired (밤 시간 코딩) ──
        (CatState::Coding, StateEvent::IdleTimeout(_)) if is_night_time() => {
            Some(CatState::Tired)
        }
        (CatState::Tired, StateEvent::ActivityDetected) => Some(CatState::Coding),

        _ => None, // 전환 없음
    }
}

/// 밤 시간 판정 (기본: 23시 ~ 6시)
fn is_night_time() -> bool {
    let hour = Local::now().hour();
    hour >= 23 || hour < 6
}

use chrono::Timelike;
