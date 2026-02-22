/// 백엔드 → 프론트엔드 이벤트 이름 상수
///
/// 이벤트 네이밍: "도메인:액션" 패턴
pub mod event_names {
    // 고양이 상태
    pub const CAT_STATE_CHANGED: &str = "cat:state-changed";
    pub const CAT_LEVEL_UP: &str = "cat:level-up";
    pub const CAT_EXP_GAINED: &str = "cat:exp-gained";

    // 활동 감지
    pub const ACTIVITY_IDE_DETECTED: &str = "activity:ide-detected";
    pub const ACTIVITY_IDLE: &str = "activity:idle";
    pub const ACTIVITY_FULLSCREEN: &str = "activity:fullscreen";

    // Git
    pub const GIT_NEW_COMMIT: &str = "git:new-commit";

    // 뽀모도로
    pub const POMODORO_TICK: &str = "pomodoro:tick";
    pub const POMODORO_COMPLETE: &str = "pomodoro:complete";
    pub const POMODORO_CANCELLED: &str = "pomodoro:cancelled";

    // 시스템
    pub const DAY_CHANGED: &str = "system:day-changed";
}
