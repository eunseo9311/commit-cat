#[tauri::command]
pub async fn check_fullscreen() -> Result<bool, String> {
    // TODO: OS별 풀스크린 감지
    // macOS: CGDisplayCapture / NSApplication
    // Windows: foreground window size vs screen size
    Ok(false)
}
