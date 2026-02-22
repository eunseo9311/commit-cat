use crate::models::settings::AppData;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

const DATA_FILE: &str = "commit-cat-data.json";

/// 앱 데이터 디렉토리 경로
fn data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    Ok(dir)
}

/// 데이터 파일 경로
fn data_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(data_dir(app)?.join(DATA_FILE))
}

/// 초기화: 데이터 파일이 없으면 기본값으로 생성
pub fn init(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let dir = data_dir(app)?;
    std::fs::create_dir_all(&dir)?;

    let path = dir.join(DATA_FILE);
    if !path.exists() {
        let default_data = AppData::default();
        let json = serde_json::to_string_pretty(&default_data)?;
        std::fs::write(&path, json)?;
    }
    Ok(())
}

/// 데이터 로드
pub fn load(app: &AppHandle) -> Result<AppData, String> {
    let path = data_path(app)?;
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read data: {}", e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse data: {}", e))
}

/// 데이터 저장
pub fn save(app: &AppHandle, data: &AppData) -> Result<(), String> {
    let path = data_path(app)?;
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize: {}", e))?;
    std::fs::write(&path, json)
        .map_err(|e| format!("Failed to write: {}", e))
}

/// History 관리: 90일 초과 데이터 정리
pub fn cleanup_history(data: &mut AppData) {
    if data.history.len() > 90 {
        data.history.truncate(90);
    }
}
