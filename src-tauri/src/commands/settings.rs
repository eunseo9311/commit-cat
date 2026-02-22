use crate::models::settings::AppSettings;

#[tauri::command]
pub async fn get_settings() -> Result<AppSettings, String> {
    Ok(AppSettings::default())
}

#[tauri::command]
pub async fn update_settings(_settings: AppSettings) -> Result<bool, String> {
    // TODO: 설정 저장
    Ok(true)
}
