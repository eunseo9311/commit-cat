/// 유틸리티 함수들

/// 오늘 날짜 문자열 (YYYY-MM-DD)
pub fn today_string() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// 분 → "Xh Ym" 포맷
pub fn format_minutes(minutes: u32) -> String {
    if minutes < 60 {
        format!("{}m", minutes)
    } else {
        format!("{}h {}m", minutes / 60, minutes % 60)
    }
}
