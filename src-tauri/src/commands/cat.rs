use crate::models::cat::CatInfo;

/// 고양이 현재 상태 조회
#[tauri::command]
pub async fn get_cat_state() -> Result<CatInfo, String> {
    // TODO: 실제 상태 머신에서 현재 상태 가져오기
    Ok(CatInfo {
        state: crate::models::cat::CatState::Idle,
        mood: crate::models::cat::CatMood::Happy,
        level: 1,
        exp: 0,
        exp_to_next: 60,
        streak_days: 0,
    })
}

/// 고양이 클릭 인터랙션
#[tauri::command]
pub async fn click_cat() -> Result<String, String> {
    // TODO: 상태를 Interaction으로 전환, 반응 애니메이션 트리거
    Ok("meow!".to_string())
}
