#[tauri::command]
pub async fn get_today_commits() -> Result<u32, String> {
    // TODO: 등록된 repo들에서 오늘 커밋 수 합산
    Ok(0)
}

#[tauri::command]
pub async fn register_repo(path: String) -> Result<bool, String> {
    // TODO: git repo 경로 등록 → settings에 저장
    // 유효한 git repo인지 확인 (.git 디렉토리 존재)
    let git_dir = std::path::Path::new(&path).join(".git");
    if git_dir.exists() {
        // TODO: settings.git_repos에 추가
        Ok(true)
    } else {
        Err("Not a valid git repository".to_string())
    }
}
